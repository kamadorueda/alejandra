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

    // x @
    let child = children.peek_next().unwrap();
    if let rnix::SyntaxKind::NODE_PAT_BIND = child.element.kind() {
        match layout {
            crate::config::Layout::Tall => {
                steps.push_back(crate::builder::Step::FormatWider(
                    child.element,
                ));
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
            crate::config::Layout::Wide => {
                steps.push_back(crate::builder::Step::Format(child.element));
                steps.push_back(crate::builder::Step::Whitespace);
            }
        }
        children.move_next();
    }

    // /**/
    children.drain_comments(|text| {
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    });

    // {
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));

    while let Some(child) = children.peek_next() {
        match child.element.kind() {
            // /**/
            rnix::SyntaxKind::TOKEN_COMMENT => {
                let prev_kind = children.peek_prev().unwrap().element.kind();
                if let rnix::SyntaxKind::TOKEN_COMMA
                | rnix::SyntaxKind::TOKEN_CURLY_B_OPEN = prev_kind
                {
                    steps.push_back(crate::builder::Step::Whitespace);
                }

                if let rnix::SyntaxKind::TOKEN_COMMENT
                | rnix::SyntaxKind::TOKEN_ELLIPSIS
                | rnix::SyntaxKind::NODE_PAT_ENTRY = prev_kind
                {
                    steps.push_back(crate::builder::Step::NewLine);
                    steps.push_back(crate::builder::Step::Pad);
                    steps.push_back(crate::builder::Step::Whitespace);
                    steps.push_back(crate::builder::Step::Whitespace);
                }

                children.drain_comment(|text| {
                    steps.push_back(crate::builder::Step::Comment(text));
                });
            }
            // item
            rnix::SyntaxKind::TOKEN_ELLIPSIS
            | rnix::SyntaxKind::NODE_PAT_ENTRY => {
                let prev_kind = children.peek_prev().unwrap().element.kind();

                if let rnix::SyntaxKind::TOKEN_COMMA
                | rnix::SyntaxKind::TOKEN_CURLY_B_OPEN = prev_kind
                {
                    steps.push_back(crate::builder::Step::Whitespace);
                }

                if let rnix::SyntaxKind::TOKEN_COMMENT = prev_kind {
                    steps.push_back(crate::builder::Step::NewLine);
                    steps.push_back(crate::builder::Step::Pad);
                    steps.push_back(crate::builder::Step::Whitespace);
                    steps.push_back(crate::builder::Step::Whitespace);
                }

                match layout {
                    crate::config::Layout::Tall => {
                        steps.push_back(crate::builder::Step::FormatWider(
                            child.element,
                        ));
                    }
                    crate::config::Layout::Wide => {
                        steps.push_back(crate::builder::Step::Format(
                            child.element,
                        ));
                    }
                };
                children.move_next();
            }
            // ,
            rnix::SyntaxKind::TOKEN_COMMA => {
                match layout {
                    crate::config::Layout::Tall => {
                        steps.push_back(crate::builder::Step::NewLine);
                        steps.push_back(crate::builder::Step::Pad);
                    }
                    crate::config::Layout::Wide => {}
                };
                steps.push_back(crate::builder::Step::Format(child.element));
                children.move_next();
            }
            _ => {
                break;
            }
        }
    }

    // }
    let child = children.get_next().unwrap();
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Whitespace);
        }
    }
    steps.push_back(crate::builder::Step::Format(child.element));

    // /**/
    children.drain_comments(|text| {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        steps.push_back(crate::builder::Step::Comment(text));
    });

    // @ x
    if let Some(child) = children.peek_next() {
        if let rnix::SyntaxKind::NODE_PAT_BIND = child.element.kind() {
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
        }
    }

    steps
}
