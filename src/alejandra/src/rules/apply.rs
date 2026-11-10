pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    let mut steps = Vec::new();

    let mut children = crate::annotated_children::annotated(build_ctx, node);

    let first = children.next().unwrap();
    let second = children.next().unwrap();

    // A one-line block comment between the function and a string, with no
    // line break, stays there: `f /* sh */ ''...''` (an editor hint)
    let hint = match first.trivialities.as_slice() {
        [crate::annotated_children::Trivia::Comment(text)]
            if !first.has_inline_comment
                && !second.has_inline_comment
                && !second.has_trivialities
                && text.starts_with("/*")
                && !text.contains('\n')
                && matches!(
                    second.element.kind(),
                    rnix::SyntaxKind::NODE_STRING
                ) =>
        {
            Some(text.clone())
        }
        _ => None,
    };

    let vertical = build_ctx.vertical
        || first.has_inline_comment
        || (first.has_trivialities && hint.is_none())
        || second.has_inline_comment
        || second.has_trivialities;

    // first
    if vertical {
        steps.push(crate::builder::Step::FormatWider(first.element));
    } else {
        steps.push(crate::builder::Step::Format(first.element));
    }

    // The line break after the comment comes with `second`
    if let Some(text) = first.inline_comment {
        steps.push(crate::builder::Step::Whitespace);
        steps.push(crate::builder::Step::Comment(text));
    }
    let hint_is_some = hint.is_some();
    if let Some(text) = hint {
        steps.push(crate::builder::Step::Whitespace);
        steps.push(crate::builder::Step::Comment(text));
    } else {
        crate::annotated_children::emit_trivialities_newline_first(
            &first.trivialities,
            &mut steps,
        );
    }

    // second
    if vertical {
        if !first.has_inline_comment
            && (!first.has_trivialities || hint_is_some)
            && matches!(
                second.element.kind(),
                rnix::SyntaxKind::NODE_ATTR_SET
                    | rnix::SyntaxKind::NODE_LIST
                    | rnix::SyntaxKind::NODE_PAREN
                    | rnix::SyntaxKind::NODE_STRING
            )
        {
            steps.push(crate::builder::Step::Whitespace);
        } else {
            steps.push(crate::builder::Step::NewLine);
            steps.push(crate::builder::Step::Pad);
        };
        steps.push(crate::builder::Step::FormatWider(second.element));
    } else {
        steps.push(crate::builder::Step::Whitespace);
        steps.push(crate::builder::Step::Format(second.element));
    }

    steps
}
