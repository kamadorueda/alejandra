pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new(build_ctx, node);

    let vertical = children.has_comments()
        || children.has_newlines()
        || build_ctx.vertical;

    while children.has_next() {
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::Comment(text));
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
            crate::children::Trivia::Whitespace(_) => {}
        });

        if let Some(child) = children.get_next() {
            if vertical {
                steps.push_back(crate::builder::Step::FormatWider(
                    child.element,
                ));
                steps.push_back(crate::builder::Step::NewLine);
            } else {
                steps.push_back(crate::builder::Step::Format(child.element));
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
