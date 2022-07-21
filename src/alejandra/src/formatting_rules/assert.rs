use std::rc::Rc;

use nixel::cst::CST;
use nixel::deps::santiago::lexer::Lexeme;

use crate::formatter::Formatter;
use crate::trivia::Trivia;
use crate::trivia::Trivialities;

pub(crate) fn rule(
    formatter: &mut Formatter,

    assert: Rc<Lexeme>,
    assert_trivia: Vec<Rc<Lexeme>>,
    expression: Box<CST>,
    expression_trivia: Vec<Rc<Lexeme>>,
    semicolon: Rc<Lexeme>,
    semicolon_trivia: Vec<Rc<Lexeme>>,
    target: Box<CST>,
) -> Result<(), ()> {
    let assert_trivia = Trivialities::from(assert_trivia);
    let expression_trivia = Trivialities::from(expression_trivia);
    let semicolon_trivia = Trivialities::from(semicolon_trivia);

    let vertical = formatter.option_vertical
        || assert_trivia.inline_comment.is_some()
        || !assert_trivia.trivialities.is_empty()
        || expression_trivia.inline_comment.is_some()
        || !expression_trivia.trivialities.is_empty()
        || semicolon_trivia.inline_comment.is_some()
        || !semicolon_trivia.trivialities.is_empty();

    formatter.add_lexeme(assert)?;

    if let Some(comment) = assert_trivia.inline_comment {
        formatter.add_ws();
        formatter.add_comment(comment);
        formatter.add_newline()?;
        formatter.add_padding();
    } else if assert_trivia.trivialities_contains_comments {
        formatter.add_newline()?;
        formatter.add_padding();
    } else {
        formatter.add_ws();
    }

    for trivia in assert_trivia.trivialities {
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

    formatter.add_lexeme(semicolon)?;

    let semicolon_trivia_has_inline_comment =
        semicolon_trivia.inline_comment.is_some();

    if let Some(comment) = semicolon_trivia.inline_comment {
        formatter.add_ws();
        formatter.add_comment(comment);
        formatter.add_newline()?;
        formatter.add_padding();
    }

    for trivia in semicolon_trivia.trivialities {
        match trivia {
            Trivia::Comment(comment) => {
                formatter.add_newline()?;
                formatter.add_padding();
                formatter.add_comment(comment);
            },
            Trivia::Newlines(_) => {},
        }
    }

    if vertical {
        if matches!(&*target, CST::Assert { .. } | CST::With { .. }) {
            formatter.add_newline()?;
            formatter.add_padding();
            formatter.format_wider(*target)?;
        } else if semicolon_trivia_has_inline_comment
            || semicolon_trivia.trivialities_contains_comments
            || !matches!(
                &*target,
                CST::Float { .. }
                    | CST::Int { .. }
                    | CST::LetIn { .. }
                    | CST::List { .. }
                    | CST::Parentheses { .. }
                    | CST::String { .. }
                    | CST::Variable { .. }
            )
        {
            formatter.indent();
            formatter.add_newline()?;
            formatter.add_padding();
            formatter.format_wider(*target)?;
            formatter.dedent();
        } else {
            formatter.add_ws();
            formatter.format_wider(*target)?;
        }
    } else {
        formatter.add_ws();
        formatter.format(*target)?;
    }

    Ok(())
}
