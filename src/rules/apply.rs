pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new(build_ctx, node);

    let layout = if children.has_comments() {
        &crate::config::Layout::Tall
    } else {
        build_ctx.config.layout()
    };

    // left
    let child = children.get_next().unwrap();
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::FormatWider(child.element));
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Format(child.element));
        }
    }

    // /**/
    children.drain_comments(|text| {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        steps.push_back(crate::builder::Step::Comment(text));
    });

    let child_prev = children.peek_prev().unwrap();

    // right
    let child = children.get_next().unwrap();
    match layout {
        crate::config::Layout::Tall => {
            if let rnix::SyntaxKind::TOKEN_COMMENT = child_prev.element.kind() {
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
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Whitespace);
            steps.push_back(crate::builder::Step::Format(child.element));
        }
    }

    steps
}
