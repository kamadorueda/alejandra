use std::rc::Rc;

use nixel::cst::ListElement;
use nixel::cst::CST;
use nixel::deps::santiago::lexer::Lexeme;

use crate::formatter::Formatter;
use crate::trivia::Trivia;
use crate::trivia::Trivialities;

pub(crate) fn rule(
    formatter: &mut Formatter,

    open: Rc<Lexeme>,
    open_trivia: Vec<Rc<Lexeme>>,
    elements: Vec<ListElement>,
    close: Rc<Lexeme>,
) -> Result<(), ()> {
    let open_trivia = Trivialities::from(open_trivia);

    let elements_count = elements.len();
    let elements: Vec<(CST, Trivialities)> = elements
        .into_iter()
        .map(|element| {
            (*element.expression, Trivialities::from(element.expression_trivia))
        })
        .collect();

    let vertical = formatter.option_vertical
        || open_trivia.inline_comment.is_some()
        || open_trivia.trivialities_contains_comments
        || open_trivia.trivialities_contains_newlines
        || elements.iter().any(|(_, trivialities)| {
            trivialities.inline_comment.is_some()
                || trivialities.trivialities_contains_comments
                || trivialities.trivialities_contains_newlines
        });

    formatter.add_lexeme(open)?;

    if vertical {
        formatter.indent();
    }

    let _inline_next_comment = false;

    for (element_index, (element, element_trivia)) in
        elements.into_iter().enumerate()
    {
        if vertical {
            formatter.add_newline()?;
            formatter.add_padding();
            formatter.format_wider(element)?;
        } else {
            if element_index >= 1 {
                formatter.add_ws();
            }
            formatter.format(element)?;
        }

        if let Some(comment) = element_trivia.inline_comment {
            formatter.add_ws();
            formatter.add_comment(comment);
            if element_trivia.trivialities.is_empty() {
                formatter.add_newline()?;
            }
        }

        for trivia in element_trivia.trivialities {
            match trivia {
                Trivia::Comment(comment) => {
                    formatter.add_newline()?;
                    formatter.add_padding();
                    formatter.add_comment(comment);
                },
                Trivia::Newlines(newlines) => {
                    if element_index + 1 < elements_count && newlines >= 2 {
                        formatter.add_newline()?;
                    }
                },
            }
        }
    }

    if vertical {
        formatter.dedent();
        formatter.add_newline()?;
        formatter.add_padding();
    }
    formatter.add_lexeme(close)?;

    Ok(())
}
