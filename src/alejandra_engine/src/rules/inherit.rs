pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let children: Vec<crate::children2::Child> =
        crate::children2::new(build_ctx, node).collect();

    let vertical = build_ctx.vertical
        || children
            .iter()
            .any(|child| child.has_inline_comment || child.has_trivialities);

    let children_count = children.len() - 1;
    let mut children = children.into_iter();

    // inherit
    let child = children.next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));
    if vertical {
        steps.push_back(crate::builder::Step::Indent);
    }

    if let Some(text) = child.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else if vertical {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    for trivia in child.trivialities {
        match trivia {
            crate::children2::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::Comment(text));
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
            crate::children2::Trivia::Newlines(_) => {}
        }
    }

    for (index, child) in children.into_iter().enumerate() {
        let not_last_child = index + 1 < children_count;

        if vertical {
            steps.push_back(crate::builder::Step::FormatWider(child.element));

            if let Some(text) = child.inline_comment {
                steps.push_back(crate::builder::Step::Whitespace);
                steps.push_back(crate::builder::Step::Comment(text));
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            } else if (not_last_child && !child.has_trivialities)
                || matches!(
                    child.trivialities.front(),
                    Some(crate::children2::Trivia::Comment(_))
                )
            {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }

            for trivia in child.trivialities {
                match trivia {
                    crate::children2::Trivia::Comment(text) => {
                        steps.push_back(crate::builder::Step::Comment(text));
                    }
                    crate::children2::Trivia::Newlines(_) => {}
                }
                if not_last_child {
                    steps.push_back(crate::builder::Step::NewLine);
                    steps.push_back(crate::builder::Step::Pad);
                }
            }
        } else {
            if not_last_child {
                steps.push_back(crate::builder::Step::Whitespace);
            }
            steps.push_back(crate::builder::Step::Format(child.element));
        }
    }

    if vertical {
        steps.push_back(crate::builder::Step::Dedent);
    }

    steps
}
