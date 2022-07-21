use std::rc::Rc;

use nixel::deps::santiago::lexer::Lexeme;

use crate::utils::count_newlines;

pub(crate) struct Trivialities {
    pub(crate) inline_comment: Option<Rc<Lexeme>>,
    pub(crate) trivialities: Vec<Trivia>,
    pub(crate) trivialities_contains_comments: bool,
    pub(crate) trivialities_contains_newlines: bool,
}

pub(crate) enum Trivia {
    Comment(Rc<Lexeme>),
    Newlines(usize),
}

impl std::convert::From<Vec<Rc<Lexeme>>> for Trivialities {
    fn from(from: Vec<Rc<Lexeme>>) -> Trivialities {
        let mut trivialities_contains_comments = false;
        let mut trivialities_contains_newlines = false;
        let mut inline_comment = None;
        let mut trivialities_copy = Vec::with_capacity(from.len());
        let mut skip_next_newline = false;

        for lexeme in &from {
            match lexeme.kind.as_str() {
                "COMMENT" => {
                    if inline_comment.is_none()
                        && trivialities_copy.is_empty()
                        && lexeme.raw.starts_with('#')
                    {
                        inline_comment = Some(lexeme.clone());
                        skip_next_newline = true;
                    } else {
                        trivialities_copy.push(Trivia::Comment(lexeme.clone()));
                        trivialities_contains_comments = true;
                    }
                },
                "WS" => {
                    let mut newlines = count_newlines(&lexeme.raw);

                    if skip_next_newline && newlines > 0 {
                        newlines -= 1;
                        skip_next_newline = false;
                    }

                    if newlines > 0 {
                        trivialities_copy.push(Trivia::Newlines(newlines));
                        trivialities_contains_newlines = true;
                    }
                },
                _ => {},
            };
        }

        Trivialities {
            inline_comment,
            trivialities: trivialities_copy,
            trivialities_contains_comments,
            trivialities_contains_newlines,
        }
    }
}
