pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new_with_configuration(
        build_ctx, node, true,
    );

    let items_count = node
        .children()
        .filter(|element| {
            matches!(
                element.kind(),
                rnix::SyntaxKind::NODE_KEY_VALUE
                    | rnix::SyntaxKind::NODE_INHERIT
                    | rnix::SyntaxKind::NODE_INHERIT_FROM
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

    // let
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::Indent);
        }
        crate::config::Layout::Wide => {}
    }

    let mut item_index: usize = 0;
    let mut inline_next_comment = false;

    loop {
        // /**/
        children.drain_comments_and_newlines(|element| match element {
            crate::children::DrainCommentOrNewline::Comment(text) => {
                if inline_next_comment && text.starts_with('#') {
                    steps.push_back(crate::builder::Step::Whitespace);
                } else {
                    steps.push_back(crate::builder::Step::NewLine);
                    steps.push_back(crate::builder::Step::Pad);
                }
                steps.push_back(crate::builder::Step::Comment(text));
                inline_next_comment = false;
            }
            crate::children::DrainCommentOrNewline::Newline(newlines) => {
                if newlines > 1 && item_index > 0 && item_index < items_count {
                    steps.push_back(crate::builder::Step::NewLine);
                }

                inline_next_comment = newlines == 0;
            }
        });

        if let Some(child) = children.peek_next() {
            if let rnix::SyntaxKind::TOKEN_IN = child.element.kind() {
                break;
            }

            // expr
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
            inline_next_comment = true;
        }
    }

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

    // in
    let child_in = children.get_next().unwrap();

    // /**/
    let mut child_comments = std::collections::LinkedList::new();
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            child_comments.push_back(crate::builder::Step::Comment(text))
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
    });

    // expr
    let child_expr = children.get_next().unwrap();

    // in
    let mut dedent = false;
    steps.push_back(crate::builder::Step::Format(child_in.element));
    match layout {
        crate::config::Layout::Tall => {
            if child_comments.is_empty()
                && matches!(
                    child_expr.element.kind(),
                    rnix::SyntaxKind::NODE_ATTR_SET
                        | rnix::SyntaxKind::NODE_LET_IN
                        | rnix::SyntaxKind::NODE_LIST
                        | rnix::SyntaxKind::NODE_PAREN
                        | rnix::SyntaxKind::NODE_STRING
                )
            {
                steps.push_back(crate::builder::Step::Whitespace);
            } else {
                dedent = true;
                steps.push_back(crate::builder::Step::Indent);
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
        }
        crate::config::Layout::Wide => {}
    }

    // /**/
    for comment in child_comments {
        steps.push_back(comment);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    // expr
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::FormatWider(
                child_expr.element,
            ));
            if dedent {
                steps.push_back(crate::builder::Step::Dedent);
            }
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Whitespace);
            steps.push_back(crate::builder::Step::Format(child_expr.element));
        }
    }

    steps
}
