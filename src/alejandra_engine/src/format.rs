#[derive(Clone)]
pub enum Status {
    Error(String),
    Changed(bool),
}

impl From<std::io::Error> for Status {
    fn from(error: std::io::Error) -> Status {
        Status::Error(error.to_string())
    }
}

pub fn in_memory(path: String, before: String) -> (Status, String) {
    let tokens = rnix::tokenizer::Tokenizer::new(&before);
    let ast = rnix::parser::parse(tokens);

    let errors = ast.errors();
    if !errors.is_empty() {
        return (Status::Error(errors[0].to_string()), before);
    }

    let after = crate::builder::build(ast.node().into(), false, path, true)
        .unwrap()
        .to_string();

    if before == after {
        (Status::Changed(false), after)
    } else {
        (Status::Changed(true), after)
    }
}

pub fn in_place(path: String) -> Status {
    use std::io::Write;

    match std::fs::read_to_string(&path) {
        Ok(before) => {
            let (status, data) = crate::format::in_memory(path.clone(), before);

            if let Status::Changed(changed) = status {
                if changed {
                    return match std::fs::File::create(path) {
                        Ok(mut file) => match file.write_all(data.as_bytes()) {
                            Ok(_) => status,
                            Err(error) => Status::from(error),
                        },
                        Err(error) => Status::from(error),
                    };
                }
            }

            status
        }
        Err(error) => Status::from(error),
    }
}
