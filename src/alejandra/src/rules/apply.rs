pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    let mut steps = Vec::new();

    let mut children = crate::annotated_children::annotated(build_ctx, node);

    let first = children.next().unwrap();
    let second = children.next().unwrap();

    let vertical = build_ctx.vertical
        || first.has_inline_comment
        || first.has_trivialities
        || second.has_inline_comment
        || second.has_trivialities;

    // first
    if vertical {
        steps.push(crate::builder::Step::FormatWider(first.element));
    } else {
        steps.push(crate::builder::Step::Format(first.element));
    }

    if let Some(text) = first.inline_comment {
        steps.push(crate::builder::Step::Whitespace);
        steps.push(crate::builder::Step::Comment(text));
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
    }

    for trivia in first.trivialities {
        match trivia {
            crate::annotated_children::Trivia::Comment(text) => {
                steps.push(crate::builder::Step::NewLine);
                steps.push(crate::builder::Step::Pad);
                steps.push(crate::builder::Step::Comment(text));
            }
            crate::annotated_children::Trivia::Newlines => {}
        }
    }

    // second
    if vertical {
        if !first.has_inline_comment
            && !first.has_trivialities
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
