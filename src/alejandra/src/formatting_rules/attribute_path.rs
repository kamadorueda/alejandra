use nixel::cst::AttributePathPart;


use crate::formatter::Formatter;

pub(crate) fn rule(
    formatter: &mut Formatter,

    parts: Vec<AttributePathPart>,
) -> Result<(), ()> {
    for part in parts.into_iter() {
        formatter.format(*part.part)?;

        if let Some(dot) = part.dot {
            formatter.add_lexeme(dot.dot)?;
        }
    }

    Ok(())
}
