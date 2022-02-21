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

    // with
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

    if comment {
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
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Format(child.element));
        }
    }

    // ;
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));

    // /**/
    let mut comment: bool = false;
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
    match layout {
        crate::config::Layout::Tall => {
            if {
                matches!(
                    child.element.kind(),
                    rnix::SyntaxKind::NODE_ASSERT | rnix::SyntaxKind::NODE_WITH
                )
            } {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::FormatWider(
                    child.element,
                ));
            } else if comment
                || !matches!(
                    child.element.kind(),
                    rnix::SyntaxKind::NODE_ATTR_SET
                        | rnix::SyntaxKind::NODE_IDENT
                        | rnix::SyntaxKind::NODE_PAREN
                        | rnix::SyntaxKind::NODE_LET_IN
                        | rnix::SyntaxKind::NODE_LIST
                        | rnix::SyntaxKind::NODE_LITERAL
                        | rnix::SyntaxKind::NODE_STRING
                )
            {
                steps.push_back(crate::builder::Step::Indent);
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::FormatWider(
                    child.element,
                ));
                steps.push_back(crate::builder::Step::Dedent);
            } else {
                steps.push_back(crate::builder::Step::Whitespace);
                steps.push_back(crate::builder::Step::FormatWider(
                    child.element,
                ));
            }
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Whitespace);
            steps.push_back(crate::builder::Step::Format(child.element));
        }
    }

    steps
}
