pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new_with_configuration(
        build_ctx, node, true,
    );

    let vertical = children.has_comments()
        || children.has_newlines()
        || build_ctx.vertical;

    // left
    let child = children.get_next().unwrap();

    if vertical {
        steps.push_back(crate::builder::Step::FormatWider(child.element));
    } else {
        steps.push_back(crate::builder::Step::Format(child.element));
    }

    // /**/
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::Comment(text));
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
    });

    let child_prev = children.peek_prev().unwrap();

    // right
    let child = children.get_next().unwrap();
    if vertical {
        if let rnix::SyntaxKind::TOKEN_COMMENT
        | rnix::SyntaxKind::TOKEN_WHITESPACE = child_prev.element.kind()
        {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        } else if let rnix::SyntaxKind::NODE_ATTR_SET
        | rnix::SyntaxKind::NODE_LIST
        | rnix::SyntaxKind::NODE_PAREN
        | rnix::SyntaxKind::NODE_STRING = child.element.kind()
        {
            steps.push_back(crate::builder::Step::Whitespace);
        } else {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        };
        steps.push_back(crate::builder::Step::FormatWider(child.element));
    } else {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Format(child.element));
    }

    steps
}
