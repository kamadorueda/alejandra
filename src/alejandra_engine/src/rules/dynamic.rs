pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new(build_ctx, node);

    let vertical = children.has_comments()
        || children.has_newlines()
        || build_ctx.vertical;

    // ${
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));
    if vertical {
        steps.push_back(crate::builder::Step::Indent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    // /**/
    children.drain_trivia(|element| match element {
        crate::children::Trivia::Comment(text) => {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        crate::children::Trivia::Whitespace(_) => {}
    });

    // expr
    let child = children.get_next().unwrap();
    if vertical {
        steps.push_back(crate::builder::Step::FormatWider(child.element));
    } else {
        steps.push_back(crate::builder::Step::Format(child.element));
    }

    // /**/
    children.drain_trivia(|element| match element {
        crate::children::Trivia::Comment(text) => {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::Comment(text));
        }
        crate::children::Trivia::Whitespace(_) => {}
    });

    // }
    let child = children.get_next().unwrap();
    if vertical {
        steps.push_back(crate::builder::Step::Dedent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }
    steps.push_back(crate::builder::Step::Format(child.element));

    steps
}
