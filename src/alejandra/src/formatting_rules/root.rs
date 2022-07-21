use std::rc::Rc;

use nixel::cst::CST;
use nixel::deps::santiago::lexer::Lexeme;

use crate::formatter::Formatter;
use crate::trivia::Trivia;
use crate::trivia::Trivialities;

pub(crate) fn rule(
    formatter: &mut Formatter,

    trivia_before: Vec<Rc<Lexeme>>,
    expression: Box<CST>,
    trivia_after: Vec<Rc<Lexeme>>,
) -> Result<(), ()> {
    let trivia_after = Trivialities::from(trivia_after);
    let trivia_before = Trivialities::from(trivia_before);

    let vertical = formatter.option_vertical
        || trivia_before.trivialities_contains_comments
        || trivia_before.trivialities_contains_newlines
        || trivia_after.trivialities_contains_comments
        || trivia_after.trivialities_contains_newlines;

    if let Some(comment) = trivia_before.inline_comment {
        formatter.add_comment(comment);
        formatter.add_newline()?;
        formatter.add_padding();
    }

    for trivia in trivia_before.trivialities {
        match trivia {
            Trivia::Comment(comment) => {
                formatter.add_comment(comment);
                formatter.add_newline()?;
                formatter.add_padding();
            },
            Trivia::Newlines(_) => {},
        }
    }

    if vertical {
        formatter.format_wider(*expression)?;
    } else {
        formatter.format(*expression)?;
    }

    if let Some(comment) = trivia_after.inline_comment {
        formatter.add_ws();
        formatter.add_comment(comment);
    }

    for trivia in trivia_after.trivialities {
        match trivia {
            Trivia::Comment(comment) => {
                formatter.add_newline()?;
                formatter.add_padding();
                formatter.add_comment(comment);
            },
            Trivia::Newlines(_) => {},
        }
    }

    formatter.add_newline()?;

    Ok(())
}
