use std::rc::Rc;

use nixel::cst::CST;
use nixel::deps::santiago::lexer::Lexeme;

use crate::formatter::Formatter;
use crate::trivia::Trivia;
use crate::trivia::Trivialities;

pub(crate) fn rule(
    formatter: &mut Formatter,

    open: Rc<Lexeme>,
    open_trivia: Vec<Rc<Lexeme>>,
    expression: Box<CST>,
    expression_trivia: Vec<Rc<Lexeme>>,
    close: Rc<Lexeme>,
) -> Result<(), ()> {
    let open_trivia = Trivialities::from(open_trivia);
    let expression_trivia = Trivialities::from(expression_trivia);

    let loose = open_trivia.inline_comment.is_some()
        || open_trivia.trivialities_contains_comments
        || expression_trivia.inline_comment.is_some()
        || expression_trivia.trivialities_contains_comments
        || matches!(&*expression, CST::IfThenElse { .. })
        || ((!open_trivia.trivialities.is_empty()
            || !expression_trivia.trivialities.is_empty())
            && !matches!(
                &*expression,
                CST::List { .. }
                    | CST::Map { .. }
                    | CST::String { .. }
                    | CST::UnaryOperation { .. }
                    | CST::Variable { .. }
            ));

    let should_indent = loose
        || matches!(
            &*expression,
            CST::FunctionApplication { .. }
                | CST::Assert { .. }
                | CST::BinaryOperation { .. }
                | CST::FunctionDestructured { .. }
                | CST::FunctionSimple { .. }
                | CST::Map { .. }
                | CST::PropertyAccess { .. }
                | CST::String { .. }
                | CST::UnaryOperation { .. }
                | CST::Variable { .. }
                | CST::With { .. }
        ) && !crate::utils::second_through_penultimate_line_are_indented(
            formatter,
            *expression.clone(),
            matches!(
                &*expression,
                CST::FunctionDestructured { .. } | CST::FunctionSimple { .. }
            ),
        );

    formatter.add_lexeme(open)?;
    if should_indent {
        formatter.indent();
    }

    if let Some(comment) = open_trivia.inline_comment {
        formatter.add_ws();
        formatter.add_comment(comment);
        formatter.add_newline()?;
        formatter.add_padding();
    } else if loose {
        formatter.add_newline()?;
        formatter.add_padding();
    }

    for trivia in open_trivia.trivialities {
        match trivia {
            Trivia::Comment(comment) => {
                formatter.add_comment(comment);
                formatter.add_newline()?;
                formatter.add_padding();
            },
            Trivia::Newlines(_) => {},
        }
    }

    if loose {
        formatter.format_wider(*expression)?;
    } else {
        formatter.format(*expression)?;
    }

    if let Some(comment) = expression_trivia.inline_comment {
        formatter.add_ws();
        formatter.add_comment(comment);
    }

    for trivia in expression_trivia.trivialities {
        match trivia {
            Trivia::Comment(comment) => {
                formatter.add_newline()?;
                formatter.add_padding();
                formatter.add_comment(comment);
            },
            Trivia::Newlines(_) => {},
        }
    }

    if should_indent {
        formatter.dedent();
    }

    if loose {
        formatter.add_newline()?;
        formatter.add_padding();
    }

    formatter.add_lexeme(close)?;

    Ok(())
}
