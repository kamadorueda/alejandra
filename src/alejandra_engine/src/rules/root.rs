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

    while children.has_next() {
        children.drain_comments_and_newlines(|element| match element {
            crate::children::DrainCommentOrNewline::Comment(text) => {
                steps.push_back(crate::builder::Step::Comment(text));
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
            crate::children::DrainCommentOrNewline::Newline(_) => {}
        });

        if let Some(child) = children.get_next() {
            match layout {
                crate::config::Layout::Tall => {
                    steps.push_back(crate::builder::Step::FormatWider(
                        child.element,
                    ));
                    steps.push_back(crate::builder::Step::NewLine);
                }
                crate::config::Layout::Wide => {
                    steps
                        .push_back(crate::builder::Step::Format(child.element));
                }
            }
        }
    }

    // Trailing newline
    if let Some(last_step) = steps.back() {
        if *last_step != crate::builder::Step::NewLine {
            steps.push_back(crate::builder::Step::NewLine);
        }
    }

    steps
}
