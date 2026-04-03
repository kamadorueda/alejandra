pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    let mut steps = Vec::new();

    let mut children = crate::children::Children::new(build_ctx, node);

    let vertical = children.has_comments()
        || children.has_newlines()
        || build_ctx.vertical;

    let child = children.get_next().unwrap();
    if vertical {
        steps.push(crate::builder::Step::FormatWider(child));
    } else {
        steps.push(crate::builder::Step::Format(child));
    }

    // /**/
    let mut comment = false;
    children.drain_trivia(|element| match element {
        crate::children::Trivia::Comment(text) => {
            steps.push(crate::builder::Step::NewLine);
            steps.push(crate::builder::Step::Pad);
            steps.push(crate::builder::Step::Comment(text));
            comment = true;
        }
        crate::children::Trivia::Whitespace(_) => {}
    });

    if comment {
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
    } else {
        steps.push(crate::builder::Step::Whitespace);
    }

    let child = children.get_next().unwrap();
    if vertical {
        steps.push(crate::builder::Step::FormatWider(child));
    } else {
        steps.push(crate::builder::Step::Format(child));
    }
    children.move_prev();

    steps
}
