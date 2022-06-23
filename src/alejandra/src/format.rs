use std::rc::Rc;

use nixel::cst::build_concrete_syntax_tree;
use nixel::deps::santiago::lexer::lex;
use nixel::deps::santiago::lexer::Lexeme;
use nixel::deps::santiago::parser::parse;
use nixel::grammar::grammar;
use nixel::lexer::lexer_rules;

use crate::formatter::Formatter;

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

/// Formats the content of `before` in-memory.
pub fn in_memory(before: String) -> (Status, String) {
    let grammar = grammar();
    let lexer_rules = lexer_rules();
    let lexemes = match lex(&lexer_rules, &before) {
        Ok(lexemes) => lexemes,
        Err(error) => {
            return (
                Status::Error(format!(
                    "At {}: {}",
                    error.position, error.message,
                )),
                before,
            );
        },
    };

    let lexemes_no_trivia: Vec<Rc<Lexeme>> = lexemes
        .iter()
        .filter(|lexeme| &lexeme.kind != "COMMENT")
        .filter(|lexeme| &lexeme.kind != "WS")
        .cloned()
        .collect();

    let parse_tree = match parse(&grammar, &lexemes_no_trivia) {
        Ok(mut parse_tree) => {
            if parse_tree.len() == 1 {
                parse_tree.remove(0)
            } else {
                return (
                    Status::Error(format!("Multiple parse trees found.")),
                    before,
                );
            }
        },
        Err(error) => {
            return (
                Status::Error(if let Some(at) = error.at {
                    format!("At {at}: Invalid Syntax")
                } else {
                    format!("Invalid Syntax")
                }),
                before,
            );
        },
    };

    let ast = parse_tree.as_abstract_syntax_tree();
    let cst = build_concrete_syntax_tree(&ast, &lexemes);

    let mut formatter = Formatter::new(false, false);
    formatter.format(cst).unwrap();

    let after = formatter.finish();

    if before == after {
        (Status::Changed(false), after)
    } else {
        (Status::Changed(true), after)
    }
}

/// Formats the file at `path`,
/// optionally overriding it's contents if `in_place` is true.
pub fn in_fs(path: String, in_place: bool) -> Status {
    use std::io::Write;

    match std::fs::read_to_string(&path) {
        Ok(before) => {
            let (status, data) = crate::format::in_memory(before);

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
                                },
                                Err(error) => Status::from(error),
                            }
                        } else {
                            Status::Changed(false)
                        }
                    } else {
                        Status::Changed(changed)
                    }
                },
                Status::Error(error) => Status::Error(error),
            }
        },
        Err(error) => Status::from(error),
    }
}
