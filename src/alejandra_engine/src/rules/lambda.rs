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

    // a
    let child = children.get_next().unwrap();
    if vertical {
        steps.push_back(crate::builder::Step::FormatWider(child.element));
    } else {
        steps.push_back(crate::builder::Step::Format(child.element));
    }

    if let rnix::SyntaxKind::TOKEN_COMMENT
    | rnix::SyntaxKind::TOKEN_WHITESPACE =
        children.peek_next().unwrap().element.kind()
    {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
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

    // :
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));

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

    // c
    let child = children.get_next().unwrap();
    if vertical {
        if comment
            || !matches!(
                child.element.kind(),
                rnix::SyntaxKind::NODE_ATTR_SET
                    | rnix::SyntaxKind::NODE_PAREN
                    | rnix::SyntaxKind::NODE_LAMBDA
                    | rnix::SyntaxKind::NODE_LET_IN
                    | rnix::SyntaxKind::NODE_LIST
                    | rnix::SyntaxKind::NODE_LITERAL
                    | rnix::SyntaxKind::NODE_STRING
            )
        {
            let should_indent = !matches!(
                child.element.kind(),
                rnix::SyntaxKind::NODE_ATTR_SET
                    | rnix::SyntaxKind::NODE_PAREN
                    | rnix::SyntaxKind::NODE_LAMBDA
                    | rnix::SyntaxKind::NODE_LET_IN
                    | rnix::SyntaxKind::NODE_LIST
                    | rnix::SyntaxKind::NODE_STRING
            ) && build_ctx.pos_new.column > 1;

            if should_indent {
                steps.push_back(crate::builder::Step::Indent);
            }

            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::FormatWider(child.element));
            if should_indent {
                steps.push_back(crate::builder::Step::Dedent);
            }
        } else {
            steps.push_back(crate::builder::Step::Whitespace);
            steps.push_back(crate::builder::Step::FormatWider(child.element));
        }
    } else {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Format(child.element));
    }

    steps
}
