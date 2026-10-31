use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use alejandra::config::Config;
use alejandra::config::Indentation;
use pretty_assertions::assert_eq;

fn configs() -> HashMap<&'static str, Config> {
    HashMap::from([
        ("default", Config::default()),
        ("indentation-four-spaces", Config {
            indentation: Indentation::FourSpaces,
            ..Default::default()
        }),
        ("indentation-tabs", Config {
            indentation: Indentation::Tabs,
            ..Default::default()
        }),
        ("preserve-one-line-attr-sets", Config {
            preserve_one_line_attr_sets: true,
            ..Default::default()
        }),
        ("space-around-brackets", Config {
            space_around_brackets: true,
            ..Default::default()
        }),
    ])
}

/// Every `(config name, config, case name, case path)`, sorted by name.
fn all_cases() -> Vec<(&'static str, Config, String, PathBuf)> {
    let cases_path = PathBuf::new().join("tests").join("cases");

    let mut all = Vec::new();

    for (config_name, config) in configs() {
        let config_cases_path = cases_path.join(config_name);

        std::fs::create_dir_all(&config_cases_path).unwrap();

        for entry in std::fs::read_dir(&config_cases_path).unwrap() {
            let case = entry.unwrap().file_name().into_string().unwrap();
            let case_path = config_cases_path.join(&case);

            all.push((config_name, config, case, case_path));
        }
    }

    all.sort_by(|a, b| (a.0, &a.2).cmp(&(b.0, &b.2)));

    all
}

fn format(path: &Path, content: &str, config: Config) -> String {
    alejandra::format::in_memory(
        path.to_str().unwrap().to_owned(),
        content.to_owned(),
        config,
    )
    .1
}

#[test]
fn cases() {
    let should_update = std::env::var("UPDATE").is_ok();

    for (_, config, _, case_path) in all_cases() {
        let path_in = case_path.join("in.nix");
        let content_in = std::fs::read_to_string(&path_in).unwrap();

        let path_out = case_path.join("out.nix");
        let content_got = format(&path_in, &content_in, config);

        if should_update {
            std::fs::File::create(&path_out)
                .unwrap()
                .write_all(content_got.as_bytes())
                .unwrap();
        }

        let content_out = std::fs::read_to_string(&path_out).unwrap();

        assert_eq!(
            content_out, content_got,
            "Test case `{:?}` failed",
            case_path
        );
    }
}

/// Asserts that the cases in `failing` are exactly the ones in `allowed`.
fn assert_failing_as_allowed(
    what: &str,
    failing: &[String],
    allowed: &[(&str, &str)],
) {
    let unexpected: Vec<&String> = failing
        .iter()
        .filter(|name| !allowed.iter().any(|(n, _)| n == name))
        .collect();
    let stale: Vec<&str> = allowed
        .iter()
        .map(|(name, _)| *name)
        .filter(|name| !failing.iter().any(|f| f == name))
        .collect();

    assert!(unexpected.is_empty(), "{what}: {unexpected:#?}");
    assert!(stale.is_empty(), "No longer {what}, remove from list: {stale:#?}");
}

/// Cases (`<config>/<case>`) whose `out.nix` changes when formatted again,
/// each with the reason. Remove an entry once it is fixed:
/// the test fails if a listed case is stable.
const NOT_IDEMPOTENT: &[(&str, &str)] = &[];

#[test]
fn cases_are_idempotent() {
    let mut failing = Vec::new();

    for (config_name, config, case, case_path) in all_cases() {
        let path_out = case_path.join("out.nix");
        let content_out = std::fs::read_to_string(&path_out).unwrap();

        if format(&path_out, &content_out, config) != content_out {
            failing.push(format!("{config_name}/{case}"));
        }
    }

    assert_failing_as_allowed("not idempotent", &failing, NOT_IDEMPOTENT);
}

/// Marks an interpolation inside the text of a string.
const INTERPOLATION: char = '\u{0}';

/// What Nix does to the content of an indented string (`''...''`):
/// drops a first line that is only spaces and strips the indentation
/// (spaces only) of the least indented line that is not only spaces.
fn strip_indentation(content: &str) -> String {
    let lines: Vec<&str> = content.split('\n').collect();
    let last = lines.len() - 1;

    let indent = |line: &str| line.len() - line.trim_start_matches(' ').len();
    let blank = |line: &str| indent(line) == line.len();

    let min_indent = lines
        .iter()
        .filter(|line| !blank(line))
        .map(|line| indent(line))
        .min()
        .unwrap_or(0);

    let mut stripped: Vec<&str> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if i == 0 && blank(line) && last > 0 {
            continue;
        }
        if blank(line) && (i == last || line.len() < min_indent) {
            stripped.push("");
        } else {
            stripped.push(&line[min_indent..]);
        }
    }
    stripped.join("\n")
}

fn is_followed_by_closing_brace(token: &rnix::SyntaxToken) -> bool {
    std::iter::successors(token.next_sibling_or_token(), |element| {
        element.next_sibling_or_token()
    })
    .find(|element| {
        !matches!(
            element.kind(),
            rnix::SyntaxKind::TOKEN_WHITESPACE
                | rnix::SyntaxKind::TOKEN_COMMENT
        )
    })
    .is_some_and(|element| element.kind() == rnix::SyntaxKind::TOKEN_R_BRACE)
}

/// Appends the non-trivia content of `node` to `out`:
/// node kinds (with nesting), token kinds and token texts.
/// The content of indented strings is compared once the indentation
/// is stripped, as Nix does.
fn normalize(node: &rnix::SyntaxNode, out: &mut Vec<String>) {
    if node.kind() == rnix::SyntaxKind::NODE_PAREN {
        if let Some(inner) = node.children().next() {
            // Alejandra removes `a = (x);` when there is nothing else inside
            let value_of_binding = std::iter::successors(
                Some(node.clone()),
                rnix::SyntaxNode::parent,
            )
            .take_while(|n| {
                n.kind() == rnix::SyntaxKind::NODE_PAREN
                    && n.children_with_tokens().count() == 3
            })
            .last()
            .and_then(|outermost| outermost.parent())
            .is_some_and(|parent| {
                parent.kind() == rnix::SyntaxKind::NODE_ATTRPATH_VALUE
            });

            // Alejandra removes parentheses that wrap a single simple
            // expression
            if value_of_binding
                || matches!(
                    inner.kind(),
                    rnix::SyntaxKind::NODE_ATTR_SET
                        | rnix::SyntaxKind::NODE_IDENT
                        | rnix::SyntaxKind::NODE_INTERPOL
                        | rnix::SyntaxKind::NODE_LIST
                        | rnix::SyntaxKind::NODE_LITERAL
                        | rnix::SyntaxKind::NODE_PAREN
                        | rnix::SyntaxKind::NODE_STRING
                )
            {
                return normalize(&inner, out);
            }
        }
    }

    let is_indented_string = node.kind() == rnix::SyntaxKind::NODE_STRING
        && node.first_token().is_some_and(|token| token.text() == "''");

    out.push(format!("({:?}", node.kind()));

    if is_indented_string {
        let mut content = String::new();
        let mut interpolations = Vec::new();

        for element in node.children_with_tokens() {
            match element {
                rnix::NodeOrToken::Token(token) => match token.kind() {
                    rnix::SyntaxKind::TOKEN_STRING_CONTENT => {
                        content.push_str(token.text());
                    }
                    kind => out.push(format!("{kind:?} {:?}", token.text())),
                },
                rnix::NodeOrToken::Node(node) => {
                    content.push(INTERPOLATION);
                    normalize(&node, &mut interpolations);
                }
            }
        }

        out.push(format!("{:?}", strip_indentation(&content)));
        out.extend(interpolations);
    } else {
        for element in node.children_with_tokens() {
            match element {
                rnix::NodeOrToken::Token(token) => match token.kind() {
                    rnix::SyntaxKind::TOKEN_WHITESPACE
                    | rnix::SyntaxKind::TOKEN_COMMENT => {}
                    // Alejandra adds a trailing comma to multiline patterns
                    rnix::SyntaxKind::TOKEN_COMMA
                        if node.kind() == rnix::SyntaxKind::NODE_PATTERN
                            && is_followed_by_closing_brace(&token) => {}
                    kind => out.push(format!("{kind:?} {:?}", token.text())),
                },
                rnix::NodeOrToken::Node(node) => normalize(&node, out),
            }
        }
    }

    out.push(")".to_string());
}

/// The normalized syntax tree, or `None` if the content does not parse.
fn syntax_tree(content: &str) -> Option<Vec<String>> {
    let parsed = rnix::Root::parse(content);

    if !parsed.errors().is_empty() {
        return None;
    }

    let mut out = Vec::new();
    normalize(&parsed.syntax(), &mut out);
    Some(out)
}

/// Cases (`<config>/<case>`) whose syntax tree changes when formatting,
/// each with the reason. Remove an entry once it is fixed:
/// the test fails if a listed case keeps its syntax tree.
const SYNTAX_TREE_CHANGES: &[(&str, &str)] = &[];

#[test]
fn cases_keep_syntax_tree() {
    let mut failing = Vec::new();

    for (config_name, _, case, case_path) in all_cases() {
        let content_in =
            std::fs::read_to_string(case_path.join("in.nix")).unwrap();
        let content_out =
            std::fs::read_to_string(case_path.join("out.nix")).unwrap();

        // Cases that do not parse are left untouched by the formatter
        let Some(tree_in) = syntax_tree(&content_in) else { continue };

        if Some(tree_in) != syntax_tree(&content_out) {
            failing.push(format!("{config_name}/{case}"));
        }
    }

    assert_failing_as_allowed(
        "syntax tree changes",
        &failing,
        SYNTAX_TREE_CHANGES,
    );
}
