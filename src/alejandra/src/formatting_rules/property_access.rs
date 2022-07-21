use std::rc::Rc;

use nixel::cst::CST;
use nixel::deps::santiago::lexer::Lexeme;

use super::binary_operation;
use super::binary_operation::Configuration;
use crate::formatter::Formatter;

pub(crate) fn rule(
    formatter: &mut Formatter,

    expression: Box<CST>,
    expression_trivia: Vec<Rc<Lexeme>>,
    dot: Rc<Lexeme>,
    dot_trivia: Vec<Rc<Lexeme>>,
    attribute_path: Box<CST>,
) -> Result<(), ()> {
    binary_operation::rule_with_configuration(
        formatter,
        expression,
        expression_trivia,
        dot,
        dot_trivia,
        attribute_path,
        Configuration::PropertyAccess,
    )
}
