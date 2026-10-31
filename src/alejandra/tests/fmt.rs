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
