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
    if vertical {
        let kind = first.element.kind();

        if matches!(kind, rnix::SyntaxKind::NODE_BIN_OP) {
            steps.push(crate::builder::Step::Format(first.element));
        } else {
            steps.push(crate::builder::Step::FormatWider(first.element));
        }
    } else {
        steps.push(crate::builder::Step::Format(first.element));
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
    if !vertical {
        steps.push(crate::builder::Step::Whitespace);
    }
    steps.push(crate::builder::Step::Format(second.element));

    crate::annotated_children::emit_inline_comment(
        &second.inline_comment,
        &mut steps,
    );

    if second.has_comments {
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
        crate::annotated_children::emit_trivialities_comment_first(
            &second.trivialities,
            &mut steps,
        );
    } else if !second.has_inline_comment {
        steps.push(crate::builder::Step::Whitespace);
    }

    // third
    if vertical {
        steps.push(crate::builder::Step::FormatWider(third.element));
    } else {
        steps.push(crate::builder::Step::Format(third.element));
    }

    steps
}
