/// Possibles results after formatting.
#[derive(Clone)]
pub enum Status {
    /// An error ocurred, and its reason.
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

/// Formats the content of `before` in-memory,
/// and assume `path` in the displayed error messages
pub fn in_memory(path: String, before: String, spaces: usize) -> (Status, String) {
    let tokens = rnix::tokenizer::Tokenizer::new(&before);
    let ast = rnix::parser::parse(tokens);

    let errors = ast.errors();
    if !errors.is_empty() {
        return (Status::Error(errors[0].to_string()), before);
    }

    let mut build_ctx = crate::builder::BuildCtx {
        force_wide: false,
        force_wide_success: true,
        indentation: 0,
        path,
        pos_old: crate::position::Position::default(),
        vertical: true,
        spaces,
    };

    let after = crate::builder::build(&mut build_ctx, ast.node().into())
        .unwrap()
        .to_string();

    if before == after {
        (Status::Changed(false), after)
    } else {
        (Status::Changed(true), after)
    }
}

/// Formats the file at `path`,
/// optionally overriding it's contents if `in_place` is true.
pub fn in_fs(path: String, in_place: bool, spaces: usize) -> Status {
    use std::io::Write;

    match std::fs::read_to_string(&path) {
        Ok(before) => {
            let (status, data) = crate::format::in_memory(path.clone(), before, spaces);

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
