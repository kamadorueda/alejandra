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
            crate::children2::Trivia::Newlines => {}
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
                // Only add padding if there are no `trivialities` (that is,
                // there's no extra `Newlines` to be added)
                // or if the first one is a comment (that is, it'll need
                // to be indented to match the content).
                if matches!(
                    child.trivialities.front(),
                    None | Some(crate::children2::Trivia::Comment(_))
                ) {
                    steps.push_back(crate::builder::Step::Pad);
                }
            } else if (not_last_child && !child.has_trivialities)
                || matches!(
                    child.trivialities.front(),
                    Some(crate::children2::Trivia::Comment(_))
                )
            {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }

            let mut trivia_iter = child.trivialities.into_iter().peekable();
            while let Some(trivia) = trivia_iter.next() {
                match trivia {
                    crate::children2::Trivia::Comment(text) => {
                        steps.push_back(crate::builder::Step::Comment(text));
                        // If the next `trivia` is a newline, don't add newlines
                        // and padding at the
                        // end of this iteration, as it will lead to a new blank
                        // line in the output.
                        if matches!(
                            trivia_iter.peek(),
                            Some(crate::children2::Trivia::Newlines)
                        ) {
                            continue;
                        }
                    }
                    crate::children2::Trivia::Newlines => {}
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
