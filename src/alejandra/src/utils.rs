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

    let range = element.text_range();
    let key = crate::builder::TrialKey {
        start:       range.start().into(),
        end:         range.end().into(),
        kind:        element.kind() as u16,
        indentation: build_ctx.indentation,
        depth:       build_ctx.fitting_in_single_line_depth,
        column:      build_ctx.pos_old.column,
        vertical:    build_ctx.vertical,
    };

    let cached = build_ctx.trial_cache.borrow().get(&key).cloned();
    let formatted = match cached {
        Some(formatted) => formatted,
        None => {
            let formatted = std::rc::Rc::new(
                crate::builder::build(&mut build_ctx, element)
                    .unwrap()
                    .to_string(),
            );
            build_ctx
                .trial_cache
                .borrow_mut()
                .insert(key, formatted.clone());
            formatted
        }
    };

    let formatted_lines: Vec<&str> = formatted.split('\n').collect();

    if formatted_lines.len() <= 2 {
        return if_leq_than_two_lines;
    }

    let indent = build_ctx.indent_str();
    let whitespace = format!("{}  ", indent);
    let lambda = format!("{}}}:", indent);
    let in_ = format!("{}in", indent);

    formatted_lines.iter().skip(1).rev().skip(1).all(|line| {
        line.is_empty()
            || line.starts_with(&lambda)
            || line.starts_with(&in_)
            || line.starts_with(&whitespace)
    })
}
