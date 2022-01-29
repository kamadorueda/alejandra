pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new(build_ctx, node);

    let layout = if children.has_comments() {
        &crate::config::Layout::Tall
    } else if node
        .children()
        .filter(|node| node.kind() == rnix::SyntaxKind::NODE_KEY_VALUE)
        .count()
        > 1
    {
        &crate::config::Layout::Tall
    } else {
        build_ctx.config.layout()
    };

    // {
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::Indent);
        }
        crate::config::Layout::Wide => {}
    }

    loop {
        // /**/
        children.drain_comments(|text| {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::Comment(text));
        });

        if let Some(child) = children.peek_next() {
            let kind = child.element.kind();

            if let rnix::SyntaxKind::TOKEN_COMMENT
            | rnix::SyntaxKind::TOKEN_CURLY_B_CLOSE = kind
            {
                break;
            }

            // item
            match layout {
                crate::config::Layout::Tall => {
                    steps.push_back(crate::builder::Step::NewLine);
                    steps.push_back(crate::builder::Step::Pad);
                    steps.push_back(crate::builder::Step::FormatWider(
                        child.element,
                    ));
                }
                crate::config::Layout::Wide => {
                    steps.push_back(crate::builder::Step::Whitespace);
                    steps
                        .push_back(crate::builder::Step::Format(child.element));
                }
            }

            children.move_next();
        } else {
            break;
        }
    }

    // /**/
    children.drain_comments(|text| {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        steps.push_back(crate::builder::Step::Comment(text));
    });

    // }
    let child = children.get_next().unwrap();
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::Dedent);
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Whitespace);
        }
    }
    steps.push_back(crate::builder::Step::Format(child.element));

    steps
}
