pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    let mut steps = Vec::new();

    let mut children = crate::annotated_children::annotated(build_ctx, node);

    let first = children.next().unwrap();
    let second = children.next().unwrap();
    let third = children.next().unwrap();

    let vertical = build_ctx.vertical
        || first.has_inline_comment
        || first.has_trivialities
        || second.has_inline_comment
        || second.has_trivialities
        || third.has_inline_comment
        || third.has_trivialities;

    // first
    steps.push(crate::builder::Step::Format(first.element));
    if vertical {
        steps.push(crate::builder::Step::Indent);
    }

    if first.inline_comment.is_some() {
        crate::annotated_children::emit_inline_comment(
            &first.inline_comment,
            &mut steps,
        );
    } else if vertical {
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
    }

    crate::annotated_children::emit_trivialities_comment_first(
        &first.trivialities,
        &mut steps,
    );

    // second
    if vertical {
        steps.push(crate::builder::Step::FormatWider(second.element));
    } else {
        steps.push(crate::builder::Step::Format(second.element));
    }

    crate::annotated_children::emit_inline_comment(
        &second.inline_comment,
        &mut steps,
    );
    crate::annotated_children::emit_trivialities_newline_first(
        &second.trivialities,
        &mut steps,
    );

    // third
    if vertical {
        steps.push(crate::builder::Step::Dedent);
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
    }
    steps.push(crate::builder::Step::Format(third.element));

    steps
}
