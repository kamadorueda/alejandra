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

/// Formats the content of `before` in-memory
/// assuming the contents come from `path` when displaying error messages
pub fn in_memory(
    path: String,
    before: String,
    config: Config,
) -> (Status, String) {
    let parsed = rnix::Root::parse(&end_comments_at_carriage_returns(&before));

    let errors = parsed.errors();

    if !errors.is_empty() {
        return (Status::Error(errors[0].to_string()), before);
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
