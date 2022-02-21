pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new_with_configuration(
        build_ctx, node, true,
    );

    let items_count = node
        .children_with_tokens()
        .filter(|element| {
            !matches!(element.kind(), rnix::SyntaxKind::TOKEN_WHITESPACE)
        })
        .count()
        - 2;

    let layout = if children.has_comments() || children.has_newlines() {
        &crate::config::Layout::Tall
    } else {
        build_ctx.config.layout()
    };

    // [
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::Indent);
        }
        crate::config::Layout::Wide => {}
    }

    let mut item_index: usize = 0;
    let mut inline_next_comment = false;

    loop {
        // /**/
        children.drain_comments_and_newlines(|element| {
            match element {
                crate::children::DrainCommentOrNewline::Comment(text) => {
                    if inline_next_comment && text.starts_with('#') {
                        steps.push_back(crate::builder::Step::Whitespace);
                    } else {
                        steps.push_back(crate::builder::Step::NewLine);
                        steps.push_back(crate::builder::Step::Pad);
                    }

                    steps.push_back(crate::builder::Step::Comment(text));
                    item_index += 1;
                    inline_next_comment = false;
                }
                crate::children::DrainCommentOrNewline::Newline(newlines) => {
                    if newlines > 1
                        && item_index > 0
                        && item_index < items_count
                    {
                        steps.push_back(crate::builder::Step::NewLine);
                    }

                    inline_next_comment = newlines == 0;
                }
            };
        });

        if let Some(child) = children.peek_next() {
            let child_kind = child.element.kind();

            if let rnix::SyntaxKind::TOKEN_SQUARE_B_CLOSE = child_kind {
                break;
            }

            // item
            item_index += 1;
            match layout {
                crate::config::Layout::Tall => {
                    steps.push_back(crate::builder::Step::NewLine);
                    steps.push_back(crate::builder::Step::Pad);
                    steps.push_back(crate::builder::Step::FormatWider(
                        child.element,
                    ));
                }
                crate::config::Layout::Wide => {
                    if item_index > 1 {
                        steps.push_back(crate::builder::Step::Whitespace);
                    }
                    steps
                        .push_back(crate::builder::Step::Format(child.element));
                }
            }

            children.move_next();
            inline_next_comment = true;
        }
    }

    // ]
    let child = children.get_next().unwrap();
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::Dedent);
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        crate::config::Layout::Wide => {}
    }
    steps.push_back(crate::builder::Step::Format(child.element));

    steps
}
