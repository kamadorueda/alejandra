pub(crate) fn has_newlines(string: &str) -> bool {
    string.chars().any(|c| c == '\n')
}

pub(crate) fn count_newlines(string: &str) -> usize {
    string.chars().filter(|c| *c == '\n').count()
}

pub(crate) fn second_through_penultimate_line_are_indented(
    build_ctx: &crate::builder::BuildCtx,
    element: rnix::SyntaxElement,
    if_leq_than_two_lines: bool,
) -> bool {
    let mut build_ctx =
        crate::builder::BuildCtx { force_wide: false, ..build_ctx.clone() };

    let formatted =
        crate::builder::build(&mut build_ctx, element).unwrap().to_string();

    let formatted_lines: Vec<&str> = formatted.split('\n').collect();

    if formatted_lines.len() <= 2 {
        return if_leq_than_two_lines;
    }

    let whitespace = format!("{0:<1$}  ", "", 2 * build_ctx.indentation);
    let lambda = format!("{0:<1$}}}:", "", 2 * build_ctx.indentation);
    let in_ = format!("{0:<1$}in", "", 2 * build_ctx.indentation);

    formatted_lines.iter().skip(1).rev().skip(1).all(|line| {
        line.is_empty()
            || line.starts_with(&lambda)
            || line.starts_with(&in_)
            || line.starts_with(&whitespace)
    })
}
