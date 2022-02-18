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

    // peek: =
    let child_equal = children.get_next().unwrap();

    // peek: /**/
    let mut comments_before = std::collections::LinkedList::new();
    let mut newlines = false;
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            comments_before.push_back(crate::builder::Step::Comment(text))
        }
        crate::children::DrainCommentOrNewline::Newline(newlines_count) => {
            if newlines_count > 0 {
                newlines = true;
            }
        }
    });

    // peek: expr
    let child_expr = children.get_next().unwrap();

    // peek: /**/
    let mut comments_after = std::collections::LinkedList::new();
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            comments_after.push_back(crate::builder::Step::Comment(text))
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
    });

    // =
    let mut dedent = false;
    steps.push_back(crate::builder::Step::Format(child_equal.element));

    match layout {
        crate::config::Layout::Tall => {
            if !comments_before.is_empty() || !comments_after.is_empty() {
                dedent = true;
                steps.push_back(crate::builder::Step::Indent);
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            } else if false
                || matches!(
                    child_expr.element.kind(),
                    rnix::SyntaxKind::NODE_ASSERT
                        | rnix::SyntaxKind::NODE_ATTR_SET
                        | rnix::SyntaxKind::NODE_PAREN
                        | rnix::SyntaxKind::NODE_LAMBDA
                        | rnix::SyntaxKind::NODE_LET_IN
                        | rnix::SyntaxKind::NODE_LIST
                        | rnix::SyntaxKind::NODE_STRING
                        | rnix::SyntaxKind::NODE_WITH
                )
                || (matches!(
                    child_expr.element.kind(),
                    rnix::SyntaxKind::NODE_APPLY
                ) && !newlines)
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
    for comment in comments_before {
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
            if !comments_after.is_empty() {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Format(child_expr.element));
        }
    }

    // /**/
    for comment in comments_after {
        steps.push_back(comment);
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
