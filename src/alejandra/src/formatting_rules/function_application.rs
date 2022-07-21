use std::rc::Rc;

use nixel::cst::FunctionApplicationArgument;
use nixel::cst::CST;
use nixel::deps::santiago::lexer::Lexeme;

use crate::formatter::Formatter;
use crate::trivia::Trivia;
use crate::trivia::Trivialities;

pub(crate) fn rule(
    formatter: &mut Formatter,

    function: Box<CST>,
    function_trivia: Vec<Rc<Lexeme>>,
    arguments: Vec<FunctionApplicationArgument>,
) -> Result<(), ()> {
    let function_trivia = Trivialities::from(function_trivia);

    let arguments: Vec<(CST, Trivialities)> = arguments
        .into_iter()
        .map(|argument| {
            (
                *argument.expression,
                Trivialities::from(argument.expression_trivia),
            )
        })
        .collect();

    let function_requires_vertical = function_trivia.inline_comment.is_some()
        || function_trivia.trivialities_contains_comments
        || function_trivia.trivialities_contains_newlines;

    let vertical = formatter.option_vertical
        || function_requires_vertical
        || arguments.iter().any(|(_, trivialities)| {
            trivialities.inline_comment.is_some()
                || trivialities.trivialities_contains_comments
                || trivialities.trivialities_contains_newlines
        });

    if vertical {
        formatter.format_wider(*function)?;
    } else {
        formatter.format(*function)?;
    }

    if let Some(comment) = function_trivia.inline_comment {
        formatter.add_ws();
        formatter.add_comment(comment);
        formatter.add_newline()?;
        formatter.add_padding();
    }

    for trivia in function_trivia.trivialities {
        match trivia {
            Trivia::Comment(comment) => {
                formatter.add_newline()?;
                formatter.add_padding();
                formatter.add_comment(comment);
            },
            Trivia::Newlines(_) => {},
        }
    }

    for (argument, _argument_trivia) in arguments.into_iter() {
        if vertical {
            if !function_requires_vertical
                && matches!(
                    &argument,
                    CST::List { .. }
                        | CST::Map { .. }
                        | CST::Parentheses { .. }
                        | CST::String { .. }
                )
            {
                formatter.add_ws();
            } else {
                formatter.add_newline()?;
                formatter.add_padding();
            }
            formatter.format_wider(argument)?;
        } else {
            formatter.add_ws();
            formatter.format(argument)?;
        }
    }

    Ok(())
}
