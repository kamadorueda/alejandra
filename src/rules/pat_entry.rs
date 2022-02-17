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

    if children.has_next() {
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

        // operator
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

        // expr
        let child = children.get_next().unwrap();
        let single_line = crate::builder::fits_in_single_line(
            build_ctx,
            child.element.clone(),
        );

        if single_line {
            if comment {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            } else {
                steps.push_back(crate::builder::Step::Whitespace);
            }
        } else {
            steps.push_back(crate::builder::Step::Indent);
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        match layout {
            crate::config::Layout::Tall => {
                steps.push_back(crate::builder::Step::FormatWider(
                    child.element,
                ));
            }
            crate::config::Layout::Wide => {
                steps.push_back(crate::builder::Step::Format(child.element));
            }
        }
        if !single_line {
            steps.push_back(crate::builder::Step::Dedent);
        }
    }

    steps
}
