use std::rc::Rc;

use nixel::cst::CST;
use nixel::deps::santiago::lexer::Lexeme;

use crate::formatter::Formatter;
use crate::trivia::Trivia;
use crate::trivia::Trivialities;

pub(crate) enum Configuration {
    BinaryOperation,
    PropertyAccess,
}

pub(crate) fn rule(
    formatter: &mut Formatter,

    left_operand: Box<CST>,
    left_operand_trivia: Vec<Rc<Lexeme>>,
    operator: Rc<Lexeme>,
    operator_trivia: Vec<Rc<Lexeme>>,
    right_operand: Box<CST>,
) -> Result<(), ()> {
    rule_with_configuration(
        formatter,
        left_operand,
        left_operand_trivia,
        operator,
        operator_trivia,
        right_operand,
        Configuration::BinaryOperation,
    )
}

pub(crate) fn rule_with_configuration(
    formatter: &mut Formatter,

    left_operand: Box<CST>,
    left_operand_trivia: Vec<Rc<Lexeme>>,
    operator: Rc<Lexeme>,
    operator_trivia: Vec<Rc<Lexeme>>,
    right_operand: Box<CST>,

    configuration: Configuration,
) -> Result<(), ()> {
    let left_operand_trivia = Trivialities::from(left_operand_trivia);
    let operator_trivia = Trivialities::from(operator_trivia);

    let vertical = formatter.option_vertical
        || left_operand_trivia.inline_comment.is_some()
        || !left_operand_trivia.trivialities.is_empty()
        || operator_trivia.inline_comment.is_some()
        || !operator_trivia.trivialities.is_empty();

    if vertical {
        if (matches!(configuration, Configuration::BinaryOperation)
            && matches!(&*left_operand, CST::BinaryOperation { .. }))
            || (matches!(configuration, Configuration::PropertyAccess)
                && matches!(&*left_operand, CST::PropertyAccess { .. }))
        {
            formatter.format(*left_operand)?;
        } else {
            formatter.format_wider(*left_operand)?;
        }
    } else {
        formatter.format(*left_operand)?;
    }

    if let Some(comment) = left_operand_trivia.inline_comment {
        formatter.add_ws();
        formatter.add_comment(comment);
        formatter.add_newline()?;
        formatter.add_padding();
    } else if vertical {
        formatter.add_newline()?;
        formatter.add_padding();
    }

    for trivia in left_operand_trivia.trivialities {
        match trivia {
            Trivia::Comment(comment) => {
                formatter.add_comment(comment);
                formatter.add_newline()?;
                formatter.add_padding();
            },
            Trivia::Newlines(_) => {},
        }
    }

    if !vertical && matches!(configuration, Configuration::BinaryOperation) {
        formatter.add_ws();
    }
    formatter.add_lexeme(operator)?;

    let operator_trivia_has_inline_comment =
        operator_trivia.inline_comment.is_some();

    if let Some(comment) = operator_trivia.inline_comment {
        formatter.add_ws();
        formatter.add_comment(comment);
        formatter.add_newline()?;
        formatter.add_padding();
    }

    if operator_trivia.trivialities_contains_comments {
        formatter.add_newline()?;
        formatter.add_padding();
        for trivia in operator_trivia.trivialities {
            match trivia {
                Trivia::Comment(comment) => {
                    formatter.add_comment(comment);
                    formatter.add_newline()?;
                    formatter.add_padding();
                },
                Trivia::Newlines(_) => {},
            }
        }
    } else if matches!(configuration, Configuration::BinaryOperation)
        && !operator_trivia_has_inline_comment
    {
        formatter.add_ws();
    }

    if vertical {
        formatter.format_wider(*right_operand)?;
    } else {
        formatter.format(*right_operand)?;
    }

    Ok(())
}
