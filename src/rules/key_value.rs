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

    // a
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

    if let rnix::SyntaxKind::TOKEN_COMMENT =
        children.peek_prev().unwrap().element.kind()
    {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else {
        steps.push_back(crate::builder::Step::Whitespace);
    }

    // =
    let mut dedent = false;
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));
    match layout {
        crate::config::Layout::Tall => {
            if let rnix::SyntaxKind::NODE_APPLY
            | rnix::SyntaxKind::NODE_ATTR_SET
            | rnix::SyntaxKind::NODE_LAMBDA
            | rnix::SyntaxKind::NODE_LIST
            | rnix::SyntaxKind::NODE_PAREN
            | rnix::SyntaxKind::NODE_STRING =
                children.peek_next().unwrap().element.kind()
            {
                steps.push_back(crate::builder::Step::Whitespace);
            } else {
                dedent = true;
                steps.push_back(crate::builder::Step::Indent);
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Whitespace);
        }
    }

    // /**/
    children.drain_comments(|text| {
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    });

    // b
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

    if let rnix::SyntaxKind::TOKEN_COMMENT =
        children.peek_prev().unwrap().element.kind()
    {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    // ;
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));
    if dedent {
        steps.push_back(crate::builder::Step::Dedent);
    }

    steps
}
