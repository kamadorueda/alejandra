pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    let mut steps = Vec::new();

    let mut children = crate::children::Children::new(build_ctx, node);

    let vertical = children.has_comments()
        || children.has_newlines()
        || build_ctx.vertical;

    // a
    let child = children.get_next().unwrap();
    if vertical {
        steps.push(crate::builder::Step::FormatWider(child));
    } else {
        steps.push(crate::builder::Step::Format(child));
    }

    if let rnix::SyntaxKind::TOKEN_COMMENT
    | rnix::SyntaxKind::TOKEN_WHITESPACE =
        children.peek_next().unwrap().kind()
    {
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
    }

    // /**/
    children.drain_trivia(|element| match element {
        crate::children::Trivia::Comment(text) => {
            steps.push(crate::builder::Step::Comment(text));
            steps.push(crate::builder::Step::NewLine);
            steps.push(crate::builder::Step::Pad);
        }
        crate::children::Trivia::Whitespace(_) => {}
    });

    // :
    let child = children.get_next().unwrap();
    steps.push(crate::builder::Step::Format(child));

    // /**/
    let mut comment = false;
    children.drain_trivia(|element| match element {
        crate::children::Trivia::Comment(text) => {
            comment = true;
            steps.push(crate::builder::Step::NewLine);
            steps.push(crate::builder::Step::Pad);
            steps.push(crate::builder::Step::Comment(text));
        }
        crate::children::Trivia::Whitespace(_) => {}
    });

    // c
    let child = children.get_next().unwrap();
    if vertical {
        if comment
            || !matches!(
                child.kind(),
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
                child.kind(),
                rnix::SyntaxKind::NODE_ATTR_SET
                    | rnix::SyntaxKind::NODE_PAREN
                    | rnix::SyntaxKind::NODE_LAMBDA
                    | rnix::SyntaxKind::NODE_LET_IN
                    | rnix::SyntaxKind::NODE_LIST
                    | rnix::SyntaxKind::NODE_STRING
            ) && build_ctx.indentation > 0;

            if should_indent {
                steps.push(crate::builder::Step::Indent);
            }

            steps.push(crate::builder::Step::NewLine);
            steps.push(crate::builder::Step::Pad);
            steps.push(crate::builder::Step::FormatWider(child));
            if should_indent {
                steps.push(crate::builder::Step::Dedent);
            }
        } else {
            steps.push(crate::builder::Step::Whitespace);
            steps.push(crate::builder::Step::FormatWider(child));
        }
    } else {
        steps.push(crate::builder::Step::Whitespace);
        steps.push(crate::builder::Step::Format(child));
    }

    steps
}
