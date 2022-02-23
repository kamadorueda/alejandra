pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new_with_configuration(
        build_ctx, node, true,
    );

    let has_comments_or_newlines =
        children.has_comments() || children.has_newlines();

    let vertical = has_comments_or_newlines || build_ctx.vertical;

    // (
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));
    if vertical && has_comments_or_newlines {
        steps.push_back(crate::builder::Step::Indent);
    }

    // /**/
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::Comment(text));
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
    });

    // expr
    let child = children.get_next().unwrap();
    if vertical {
        if has_comments_or_newlines {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        steps.push_back(crate::builder::Step::FormatWider(child.element));
    } else {
        steps.push_back(crate::builder::Step::Format(child.element));
    }

    // /**/
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::Comment(text));
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
    });

    // )
    let child = children.get_next().unwrap();
    if vertical && has_comments_or_newlines {
        steps.push_back(crate::builder::Step::Dedent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }
    steps.push_back(crate::builder::Step::Format(child.element));

    steps
}
