use std::rc::Rc;

use nixel::cst::Binding;
use nixel::cst::MapRecursive;
use nixel::cst::CST;
use nixel::deps::santiago::lexer::Lexeme;

use crate::formatter::Formatter;
use crate::trivia::Trivia;
use crate::trivia::Trivialities;

pub(crate) fn rule(
    formatter: &mut Formatter,

    recursive: Option<MapRecursive>,
    open: Rc<Lexeme>,
    open_trivia: Vec<Rc<Lexeme>>,
    bindings: Vec<Binding>,
    close: Rc<Lexeme>,
) -> Result<(), ()> {
    let bindings: Vec<(CST, Trivialities)> = bindings
        .into_iter()
        .map(|binding| {
            (binding.binding, Trivialities::from(binding.binding_trivia))
        })
        .collect();
    let bindings_count = bindings.len();

    let open_trivia = Trivialities::from(open_trivia);

    let vertical = formatter.option_vertical
        || open_trivia.inline_comment.is_some()
        || open_trivia.trivialities_contains_comments
        || open_trivia.trivialities_contains_newlines
        || bindings.iter().any(|(_, trivialities)| {
            trivialities.inline_comment.is_some()
                || trivialities.trivialities_contains_comments
                || trivialities.trivialities_contains_newlines
        });

    if let Some(recursive) = recursive {
        formatter.add_lexeme(recursive.rec);
        formatter.add_ws();
    }

    formatter.add_lexeme(open);

    if vertical {
        formatter.indent();
    }

    if let Some(comment) = open_trivia.inline_comment {
        formatter.add_ws();
        formatter.add_comment(comment);
    }

    for trivia in open_trivia.trivialities {
        match trivia {
            Trivia::Comment(comment) => {
                formatter.add_newline()?;
                formatter.add_padding();
                formatter.add_comment(comment);
            },
            Trivia::Newlines(_) => {},
        }
    }

    for (element_index, (binding, binding_trivia)) in
        bindings.into_iter().enumerate()
    {
        if vertical {
            formatter.add_newline()?;
            formatter.add_padding();
            formatter.format_wider(binding)?;
        } else {
            if element_index >= 1 {
                formatter.add_ws();
            }
            formatter.format(binding)?;
        }

        if let Some(comment) = binding_trivia.inline_comment {
            formatter.add_ws();
            formatter.add_comment(comment);
            if binding_trivia.trivialities.is_empty() {
                formatter.add_newline()?;
            }
        }

        for trivia in binding_trivia.trivialities {
            match trivia {
                Trivia::Comment(comment) => {
                    formatter.add_newline()?;
                    formatter.add_padding();
                    formatter.add_comment(comment);
                },
                Trivia::Newlines(newlines) => {
                    if element_index + 1 < bindings_count && newlines >= 2 {
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

    formatter.add_lexeme(close);

    Ok(())
}
