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

    // expr
    let child = children.get_next().unwrap();
    match layout {
        crate::config::Layout::Tall => {
            match child.element.kind() {
                rnix::SyntaxKind::NODE_OR_DEFAULT => {
                    steps
                        .push_back(crate::builder::Step::Format(child.element));
                }
                _ => {
                    steps.push_back(crate::builder::Step::FormatWider(
                        child.element,
                    ));
                }
            }
            steps.push_back(crate::builder::Step::Indent);
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

    // operator
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));

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

    // expr
    let child = children.get_next().unwrap();
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::FormatWider(child.element));
            steps.push_back(crate::builder::Step::Dedent);
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Format(child.element));
        }
    }

    steps
}
