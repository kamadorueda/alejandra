pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new(build_ctx, node);

    let vertical = children.has_comments()
        || children.has_newlines()
        || build_ctx.vertical;

    // expr
    let child = children.get_next().unwrap();
    if vertical {
        steps.push_back(crate::builder::Step::FormatWider(child));
    } else {
        steps.push_back(crate::builder::Step::Format(child));
    }

    if children.has_next() {
        // /**/
        let mut comment = false;
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::Comment(text));
                comment = true;
            }
            crate::children::Trivia::Whitespace(_) => {}
        });

        if comment {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        } else {
            steps.push_back(crate::builder::Step::Whitespace);
        }

        // operator
        let child = children.get_next().unwrap();
        steps.push_back(crate::builder::Step::Format(child));

        // /**/
        let mut comment = false;
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::Comment(text));
                comment = true;
            }
            crate::children::Trivia::Whitespace(_) => {}
        });

        // expr
        let child = children.get_next().unwrap();
        let mut dedent = false;

        if comment {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        } else if matches!(
            child.kind(),
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
            child.clone(),
        ) {
            steps.push_back(crate::builder::Step::Whitespace);
        } else {
            dedent = true;
            steps.push_back(crate::builder::Step::Indent);
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }

        if vertical {
            steps.push_back(crate::builder::Step::FormatWider(child));
        } else {
            steps.push_back(crate::builder::Step::Format(child));
        }
        if dedent {
            steps.push_back(crate::builder::Step::Dedent);
        }
    }

    steps
}
