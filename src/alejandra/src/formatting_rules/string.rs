use std::rc::Rc;

use nixel::cst::CST;
use nixel::deps::santiago::lexer::Lexeme;

use crate::formatter::Formatter;

pub(crate) fn rule(
    formatter: &mut Formatter,

    open: Rc<Lexeme>,
    parts: Vec<CST>,
    close: Rc<Lexeme>,
) -> Result<(), ()> {
    formatter.add_lexeme(open)?;
    for part in parts.into_iter() {
        formatter.format(part)?;
    }
    formatter.add_lexeme(close)?;

    Ok(())
}
