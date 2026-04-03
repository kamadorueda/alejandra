pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    let mut steps = Vec::new();

    let mut children = crate::annotated_children::annotated(build_ctx, node);

    let first = children.next().unwrap();
    let second = children.next().unwrap();
    let third = children.next().unwrap();
    let fourth = children.next().unwrap();

    let vertical = build_ctx.vertical
        || first.has_inline_comment
        || first.has_trivialities
        || second.has_inline_comment
        || second.has_trivialities
        || third.has_inline_comment
        || third.has_trivialities
        || fourth.has_inline_comment
        || fourth.has_trivialities;

    // first
    steps.push(crate::builder::Step::Format(first.element));

    if first.inline_comment.is_some() {
        crate::annotated_children::emit_inline_comment(
            &first.inline_comment,
            &mut steps,
        );
    } else if first.has_comments {
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
    } else {
        steps.push(crate::builder::Step::Whitespace);
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

    // third
    steps.push(crate::builder::Step::Format(third.element));

    crate::annotated_children::emit_inline_comment(
        &third.inline_comment,
        &mut steps,
    );
    crate::annotated_children::emit_trivialities_newline_first(
        &third.trivialities,
        &mut steps,
    );

    // fourth
    if vertical {
        if matches!(
            fourth.element.kind(),
            rnix::SyntaxKind::NODE_ASSERT | rnix::SyntaxKind::NODE_WITH
        ) {
            steps.push(crate::builder::Step::NewLine);
            steps.push(crate::builder::Step::Pad);
            steps.push(crate::builder::Step::FormatWider(fourth.element));
        } else if third.has_inline_comment
            || third.has_comments
            || !matches!(
                fourth.element.kind(),
                rnix::SyntaxKind::NODE_ATTR_SET
                    | rnix::SyntaxKind::NODE_IDENT
                    | rnix::SyntaxKind::NODE_PAREN
                    | rnix::SyntaxKind::NODE_LET_IN
                    | rnix::SyntaxKind::NODE_LIST
                    | rnix::SyntaxKind::NODE_LITERAL
                    | rnix::SyntaxKind::NODE_STRING
            )
        {
            steps.push(crate::builder::Step::Indent);
            steps.push(crate::builder::Step::NewLine);
            steps.push(crate::builder::Step::Pad);
            steps.push(crate::builder::Step::FormatWider(fourth.element));
            steps.push(crate::builder::Step::Dedent);
        } else {
            steps.push(crate::builder::Step::Whitespace);
            steps.push(crate::builder::Step::FormatWider(fourth.element));
        }
    } else {
        steps.push(crate::builder::Step::Whitespace);
        steps.push(crate::builder::Step::Format(fourth.element));
    }

    steps
}
