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

    // inherit
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));
    if vertical {
        steps.push_back(crate::builder::Step::Indent);
    }

    loop {
        // /**/
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::Comment(text));
            }
            crate::children::Trivia::Whitespace(_) => {}
        });

        if let Some(child) = children.get_next() {
            // expr
            if vertical {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::FormatWider(
                    child.element,
                ));
            } else {
                if let rnix::SyntaxKind::TOKEN_SEMICOLON = child.element.kind()
                {
                } else {
                    steps.push_back(crate::builder::Step::Whitespace);
                }
                steps.push_back(crate::builder::Step::Format(child.element));
            }
        } else {
            break;
        }
    }

    if vertical {
        steps.push_back(crate::builder::Step::Dedent);
    }

    steps
}
