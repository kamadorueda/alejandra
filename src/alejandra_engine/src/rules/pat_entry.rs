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

    // expr
    let child = children.get_next().unwrap();
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::FormatWider(child.element));
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Format(child.element));
        }
    }

    if children.has_next() {
        // /**/
        let mut comment = false;
        children.drain_comments_and_newlines(|element| match element {
            crate::children::DrainCommentOrNewline::Comment(text) => {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::Comment(text));
                comment = true;
            }
            crate::children::DrainCommentOrNewline::Newline(_) => {}
        });

        if comment {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        } else {
            steps.push_back(crate::builder::Step::Whitespace);
        }

        // operator
        let child = children.get_next().unwrap();
        steps.push_back(crate::builder::Step::Format(child.element));

        // /**/
        let mut comment = false;
        children.drain_comments_and_newlines(|element| match element {
            crate::children::DrainCommentOrNewline::Comment(text) => {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::Comment(text));
                comment = true;
            }
            crate::children::DrainCommentOrNewline::Newline(_) => {}
        });

        // expr
        let child = children.get_next().unwrap();
        let mut dedent = false;

        if comment {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        } else if matches!(
            child.element.kind(),
            rnix::SyntaxKind::NODE_ATTR_SET
                | rnix::SyntaxKind::NODE_IDENT
                | rnix::SyntaxKind::NODE_PAREN
                | rnix::SyntaxKind::NODE_LAMBDA
                | rnix::SyntaxKind::NODE_LET_IN
                | rnix::SyntaxKind::NODE_LIST
                | rnix::SyntaxKind::NODE_LITERAL
                | rnix::SyntaxKind::NODE_STRING,
        ) || crate::builder::fits_in_single_line(
            build_ctx,
            child.element.clone(),
        ) {
            steps.push_back(crate::builder::Step::Whitespace);
        } else {
            dedent = true;
            steps.push_back(crate::builder::Step::Indent);
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }

        match layout {
            crate::config::Layout::Tall => {
                steps.push_back(crate::builder::Step::FormatWider(
                    child.element,
                ));
            }
            crate::config::Layout::Wide => {
                steps.push_back(crate::builder::Step::Format(child.element));
            }
        }
        if dedent {
            steps.push_back(crate::builder::Step::Dedent);
        }
    }

    steps
}
