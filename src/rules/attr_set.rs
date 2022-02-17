pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new_with_configuration(
        build_ctx, node, true,
    );

    let items_count = node
        .children_with_tokens()
        .skip_while(|element| {
            element.kind() != rnix::SyntaxKind::TOKEN_CURLY_B_OPEN
        })
        .take_while(|element| {
            element.kind() != rnix::SyntaxKind::TOKEN_CURLY_B_CLOSE
        })
        .filter(|element| {
            matches!(
                element.kind(),
                rnix::SyntaxKind::NODE_KEY_VALUE
                    | rnix::SyntaxKind::NODE_INHERIT
                    | rnix::SyntaxKind::NODE_INHERIT_FROM
                    | rnix::SyntaxKind::TOKEN_COMMENT
            )
        })
        .count();

    let layout = if items_count > 1
        || children.has_comments()
        || children.has_newlines()
    {
        &crate::config::Layout::Tall
    } else {
        build_ctx.config.layout()
    };

    // rec
    let child = children.peek_next().unwrap();
    if let rnix::SyntaxKind::TOKEN_REC = child.element.kind() {
        steps.push_back(crate::builder::Step::Format(child.element));
        children.move_next();

        if let rnix::SyntaxKind::TOKEN_COMMENT
        | rnix::SyntaxKind::TOKEN_WHITESPACE =
            children.peek_next().unwrap().element.kind()
        {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        } else {
            steps.push_back(crate::builder::Step::Whitespace);
        }
    }

    // /**/
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
    });

    // {
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::Indent);
        }
        crate::config::Layout::Wide => {}
    }

    let mut item_index: usize = 0;

    loop {
        // /**/
        children.drain_comments_and_newlines(|element| match element {
            crate::children::DrainCommentOrNewline::Comment(text) => {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::Comment(text));
                item_index += 1;
            }
            crate::children::DrainCommentOrNewline::Newline(_) => {
                if item_index > 0 && item_index < items_count {
                    steps.push_back(crate::builder::Step::NewLine);
                }
            }
        });

        if let Some(child) = children.peek_next() {
            if let rnix::SyntaxKind::TOKEN_CURLY_B_CLOSE = child.element.kind()
            {
                break;
            }

            // item
            item_index += 1;
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
        }
    }

    // }
    let child = children.get_next().unwrap();
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::Dedent);
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        crate::config::Layout::Wide => {
            if items_count > 0 {
                steps.push_back(crate::builder::Step::Whitespace);
            }
        }
    }
    steps.push_back(crate::builder::Step::Format(child.element));

    steps
}
