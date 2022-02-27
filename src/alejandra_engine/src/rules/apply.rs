pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let parsed = crate::parsers::apply::Apply::parse(build_ctx, node);

    let vertical = build_ctx.vertical
        || !parsed.comments_after_left.is_empty()
        || parsed.has_newlines_after_left;

    // left_expression
    if vertical {
        steps.push_back(crate::builder::Step::FormatWider(
            parsed.left_expression,
        ));
    } else {
        steps.push_back(crate::builder::Step::Format(parsed.left_expression));
    }

    // comments_after_left
    let has_comments_after_left = !parsed.comments_after_left.is_empty();
    for text in parsed.comments_after_left {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        steps.push_back(crate::builder::Step::Comment(text));
    }

    // right_expression
    if vertical {
        if !has_comments_after_left
            && !parsed.has_newlines_after_left
            && matches!(
                parsed.right_expression.kind(),
                rnix::SyntaxKind::NODE_ATTR_SET
                    | rnix::SyntaxKind::NODE_LIST
                    | rnix::SyntaxKind::NODE_PAREN
                    | rnix::SyntaxKind::NODE_STRING
            )
        {
            steps.push_back(crate::builder::Step::Whitespace);
        } else {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        };
        steps.push_back(crate::builder::Step::FormatWider(
            parsed.right_expression,
        ));
    } else {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Format(parsed.right_expression));
    }

    steps
}
