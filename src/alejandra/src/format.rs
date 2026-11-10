use crate::config::Config;

/// Possibles results after formatting.
#[derive(Clone)]
pub enum Status {
    /// An error occurred, and its reason.
    Error(String),
    /// Formatting was successful,
    /// the file changed or not according to the boolean.
    Changed(bool),
}

impl From<std::io::Error> for Status {
    fn from(error: std::io::Error) -> Status {
        Status::Error(error.to_string())
    }
}

/// Nix ends a `#` comment at a lone carriage return (`\r`), rnix does not.
/// Replacing it by a line feed (also whitespace in Nix) is equivalent.
fn end_comments_at_carriage_returns(code: &str) -> std::borrow::Cow<'_, str> {
    let is_lone = |text: &str, index: usize| {
        text.as_bytes()[index] == b'\r'
            && text.as_bytes().get(index + 1) != Some(&b'\n')
    };

    if !(0..code.len()).any(|index| is_lone(code, index)) {
        return code.into();
    }

    let mut fixed = code.to_string();
    loop {
        // What follows the carriage return is code, so tokenize again
        let mut offset = 0;
        let mut found = None;
        for (kind, text) in rnix::tokenize(&fixed) {
            if kind == rnix::SyntaxKind::TOKEN_COMMENT && text.starts_with('#')
            {
                found = (0..text.len())
                    .find(|index| is_lone(text, *index))
                    .map(|index| offset + index);
                if found.is_some() {
                    break;
                }
            }
            offset += text.len();
        }

        match found {
            Some(index) => fixed.replace_range(index..index + 1, "\n"),
            None => break,
        }
    }
    fixed.into()
}

/// The line and column (both from 1) of the byte `offset` of `code`.
/// The column counts characters, not bytes.
fn line_and_column(code: &str, offset: usize) -> (usize, usize) {
    let before = &code[..offset];
    let line = before.matches('\n').count() + 1;
    let column = match before.rfind('\n') {
        Some(newline) => before[newline + 1..].chars().count() + 1,
        None => before.chars().count() + 1,
    };
    (line, column)
}

/// The byte offset where the last token before `offset` ends,
/// not counting whitespace and comments.
fn end_of_token_before(code: &str, offset: usize) -> Option<usize> {
    let mut position = 0;
    let mut end = None;
    for (kind, text) in rnix::tokenize(code) {
        if position + text.len() > offset {
            break;
        }
        position += text.len();
        if !matches!(
            kind,
            rnix::SyntaxKind::TOKEN_WHITESPACE
                | rnix::SyntaxKind::TOKEN_COMMENT
        ) {
            end = Some(position);
        }
    }
    end
}

/// The message of a parse error, with the line and column where it starts
/// in place of the byte range (`at 12..15`), which is hard to find.
fn describe_error(code: &str, error: &rnix::ParseError) -> String {
    use rnix::ParseError;

    let message = error.to_string();
    let range = match error {
        ParseError::Unexpected(range)
        | ParseError::UnexpectedExtra(range)
        | ParseError::UnexpectedWanted(_, range, _)
        | ParseError::UnexpectedDoubleBind(range)
        | ParseError::DuplicatedArgs(range, _) => *range,
        _ => return message,
    };

    let start = usize::from(range.start());
    let end = usize::from(range.end());
    if end > code.len() || !code.is_char_boundary(start) {
        return message;
    }

    let (line, column) = line_and_column(code, start);
    let mut position = format!("at line {line}, column {column}");

    // A missing `;` is often at the end of a line, but the parser sees it
    // only later, on the next line: also say where the code before that
    // line ends
    let wants_semicolon = matches!(
        error,
        ParseError::UnexpectedWanted(_, _, kinds)
            if kinds.contains(&rnix::SyntaxKind::TOKEN_SEMICOLON)
    );
    let line_start = code[..start].rfind('\n').map_or(0, |index| index + 1);
    if wants_semicolon {
        if let Some((last_char, _)) = end_of_token_before(code, line_start)
            .and_then(|end_before| code[..end_before].char_indices().last())
        {
            let (line_before, column_before) = line_and_column(code, last_char);
            position.push_str(&format!(
                " (after line {line_before}, column {column_before})"
            ));
        }
    }

    // rnix writes `at 12..15`, or `in 12..15` for duplicated arguments
    for word in ["at", "in"] {
        let range = format!("{word} {start}..{end}");
        if message.contains(&range) {
            return message.replacen(&range, &position, 1);
        }
    }
    message
}

/// Formats the content of `before` in-memory
/// assuming the contents come from `path` when displaying error messages
pub fn in_memory(
    path: String,
    before: String,
    config: Config,
) -> (Status, String) {
    let code = end_comments_at_carriage_returns(&before);
    let parsed = rnix::Root::parse(&code);

    let errors = parsed.errors();

    if !errors.is_empty() {
        return (Status::Error(describe_error(&code, &errors[0])), before);
    }

    let mut build_ctx = crate::builder::BuildCtx {
        config,
        fitting_in_single_line_depth: 0,
        force_wide: false,
        force_wide_success: true,
        indentation: 0,
        path,
        pos_old: crate::position::Position::default(),
        vertical: true,
        trial_cache: Default::default(),
    };

    let root = parsed.syntax();

    let after =
        crate::builder::build(&mut build_ctx, root.into()).unwrap().to_string();

    if before == after {
        (Status::Changed(false), after)
    } else {
        (Status::Changed(true), after)
    }
}

/// Formats the file at `path`,
/// optionally overriding it's contents if `in_place` is true.
pub fn in_fs(path: String, config: Config, in_place: bool) -> Status {
    use std::io::Write;

    match std::fs::read_to_string(&path) {
        Ok(before) => {
            let (status, data) =
                crate::format::in_memory(path.clone(), before, config);

            match status {
                Status::Changed(changed) => {
                    if in_place {
                        if changed {
                            match std::fs::File::create(path) {
                                Ok(mut file) => {
                                    match file.write_all(data.as_bytes()) {
                                        Ok(_) => Status::Changed(true),
                                        Err(error) => Status::from(error),
                                    }
                                }
                                Err(error) => Status::from(error),
                            }
                        } else {
                            Status::Changed(false)
                        }
                    } else {
                        Status::Changed(changed)
                    }
                }
                Status::Error(error) => Status::Error(error),
            }
        }
        Err(error) => Status::from(error),
    }
}
