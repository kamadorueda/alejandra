use std::rc::Rc;

use nixel::cst::CST;
use nixel::deps::santiago::lexer::Lexeme;

use crate::formatter::Formatter;
use crate::trivia::Trivialities;
use crate::utils::second_through_penultimate_line_are_indented;

pub(crate) fn rule(
    formatter: &mut Formatter,

    from: Box<CST>,
    from_trivia: Vec<Rc<Lexeme>>,
    equal: Rc<Lexeme>,
    equal_trivia: Vec<Rc<Lexeme>>,
    to: Box<CST>,
    to_trivia: Vec<Rc<Lexeme>>,
    semicolon: Rc<Lexeme>,
) -> Result<(), ()> {
    let from_trivia = Trivialities::from(from_trivia);
    let equal_trivia = Trivialities::from(equal_trivia);
    let to_trivia = Trivialities::from(to_trivia);

    let vertical = from_trivia.inline_comment.is_some()
        || !from_trivia.trivialities.is_empty()
        || equal_trivia.inline_comment.is_some()
        || !equal_trivia.trivialities.is_empty()
        || to_trivia.inline_comment.is_some()
        || !to_trivia.trivialities.is_empty();

    formatter.format(*from)?;
    formatter.add_ws();
    formatter.add_lexeme(equal)?;

    let mut dedent = false;
    if vertical {
        if equal_trivia.trivialities_contains_comments
            || to_trivia.trivialities_contains_comments
        {
            dedent = true;
            formatter.indent();
            formatter.add_newline()?;
            formatter.add_padding();
        } else if matches!(
            &*to,
            CST::Assert { .. }
                | CST::Map { .. }
                | CST::Parentheses { .. }
                | CST::FunctionDestructured { .. }
                | CST::FunctionSimple { .. }
                | CST::LetIn { .. }
                | CST::List { .. }
                | CST::String { .. }
                | CST::With { .. }
        ) || matches!(&*to, CST::FunctionApplication { .. })
            && second_through_penultimate_line_are_indented(
                formatter,
                *to.clone(),
                false,
            )
        {
            formatter.add_ws();
        } else {
            dedent = true;
            formatter.indent();
            formatter.add_newline()?;
            formatter.add_padding();
        }
    } else {
        formatter.add_ws();
    }

    if vertical {
        formatter.format_wider(*to)?;
        if to_trivia.trivialities_contains_comments {
            formatter.add_newline()?;
            formatter.add_padding();
        }
    } else {
        formatter.format(*to)?;
    }

    formatter.add_lexeme(semicolon)?;

    if dedent {
        formatter.dedent();
    }

    Ok(())
}
