pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new_with_configuration(
        build_ctx, node, true,
    );

    let layout = if children.has_comments() || children.has_newlines() {
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
    let mut comment = false;
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            comment = true;
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::Comment(text));
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
    });
    if comment {
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
            let next = children.peek_next().unwrap();
            let next_kind = next.element.kind();

            if matches!(
                next_kind,
                rnix::SyntaxKind::NODE_ATTR_SET
                    | rnix::SyntaxKind::NODE_PAREN
                    | rnix::SyntaxKind::NODE_LET_IN
                    | rnix::SyntaxKind::NODE_LIST
                    | rnix::SyntaxKind::NODE_STRING
            ) || (matches!(next_kind, rnix::SyntaxKind::NODE_LAMBDA)
                && !matches!(
                    next.element
                        .clone()
                        .into_node()
                        .unwrap()
                        .children()
                        .next()
                        .unwrap()
                        .kind(),
                    rnix::SyntaxKind::NODE_PATTERN
                ))
                || (matches!(next_kind, rnix::SyntaxKind::NODE_APPLY)
                    && matches!(
                        next.element
                            .clone()
                            .into_node()
                            .unwrap()
                            .children()
                            .collect::<Vec<rnix::SyntaxNode>>()
                            .iter()
                            .rev()
                            .next()
                            .unwrap()
                            .kind(),
                        rnix::SyntaxKind::NODE_ATTR_SET
                            | rnix::SyntaxKind::NODE_PAREN
                            | rnix::SyntaxKind::NODE_LIST
                            | rnix::SyntaxKind::NODE_STRING
                    ))
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
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
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
    let mut comment = false;
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            comment = true;
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::Comment(text));
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
    });
    if comment {
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
