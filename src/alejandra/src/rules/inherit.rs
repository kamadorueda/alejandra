pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    let mut steps = Vec::new();

    let children: Vec<crate::annotated_children::Child> =
        crate::annotated_children::annotated(build_ctx, node).collect();

    let vertical = build_ctx.vertical
        || children
            .iter()
            .any(|child| child.has_inline_comment || child.has_trivialities);

    // inherit (source {
    //   ...
    // }) a b c;
    let hanging = vertical && is_hanging(build_ctx, node, &children);
    let vertical = vertical && !hanging;

    let children_count = children.len() - 1;
    let mut children = children.into_iter();

    // inherit
    let child = children.next().unwrap();
    steps.push(crate::builder::Step::Format(child.element));
    if vertical {
        steps.push(crate::builder::Step::Indent);
    }

    if child.inline_comment.is_some() {
        crate::annotated_children::emit_inline_comment(
            &child.inline_comment,
            &mut steps,
        );
    } else if vertical {
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
    }

    crate::annotated_children::emit_trivialities_comment_first(
        &child.trivialities,
        &mut steps,
    );

    for (index, child) in children.enumerate() {
        let not_last_child = index + 1 < children_count;

        if vertical {
            steps.push(crate::builder::Step::FormatWider(child.element));

            if let Some(text) = child.inline_comment {
                steps.push(crate::builder::Step::Whitespace);
                steps.push(crate::builder::Step::Comment(text));
                steps.push(crate::builder::Step::NewLine);
                // Only add padding if there are no `trivialities` (that is,
                // there's no extra `Newlines` to be added)
                // or if the first one is a comment (that is, it'll need
                // to be indented to match the content).
                if matches!(
                    child.trivialities.first(),
                    None | Some(crate::annotated_children::Trivia::Comment(_))
                ) {
                    steps.push(crate::builder::Step::Pad);
                }
            } else if (not_last_child && !child.has_trivialities)
                || matches!(
                    child.trivialities.first(),
                    Some(crate::annotated_children::Trivia::Comment(_))
                )
            {
                steps.push(crate::builder::Step::NewLine);
                steps.push(crate::builder::Step::Pad);
            }

            let mut trivia_iter = child.trivialities.into_iter().peekable();
            while let Some(trivia) = trivia_iter.next() {
                match trivia {
                    crate::annotated_children::Trivia::Comment(text) => {
                        steps.push(crate::builder::Step::Comment(text));
                        // If the next `trivia` is a newline, don't add newlines
                        // and padding at the
                        // end of this iteration, as it will lead to a new blank
                        // line in the output.
                        if matches!(
                            trivia_iter.peek(),
                            Some(crate::annotated_children::Trivia::Newlines)
                        ) {
                            continue;
                        }
                    }
                    crate::annotated_children::Trivia::Newlines => {}
                }
                if not_last_child {
                    steps.push(crate::builder::Step::NewLine);
                    steps.push(crate::builder::Step::Pad);
                }
            }
        } else {
            if not_last_child {
                steps.push(crate::builder::Step::Whitespace);
            }
            if hanging && index == 0 {
                steps.push(crate::builder::Step::FormatWider(child.element));
            } else {
                steps.push(crate::builder::Step::Format(child.element));
            }
        }
    }

    if vertical {
        steps.push(crate::builder::Step::Dedent);
    }

    steps
}

/// Whether the inherit has a source in parentheses that hangs
/// (like an attribute set), has no comments, and the names fit
/// in the line of the closing bracket of the source.
fn is_hanging(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
    children: &[crate::annotated_children::Child],
) -> bool {
    // inherit, source, name+, ;
    if children.len() < 4
        || children[1].element.kind() != rnix::SyntaxKind::NODE_INHERIT_FROM
        || node
            .descendants_with_tokens()
            .any(|element| element.kind() == rnix::SyntaxKind::TOKEN_COMMENT)
    {
        return false;
    }

    let Some(last_line_width) = crate::utils::hanging_last_line_width(
        build_ctx,
        children[1].element.clone(),
    ) else {
        return false;
    };

    let mut width = last_line_width + 1;
    for (index, child) in children[2..children.len() - 1].iter().enumerate() {
        if child.element.kind() != rnix::SyntaxKind::NODE_IDENT {
            return false;
        }
        match crate::utils::single_line_width(build_ctx, child.element.clone())
        {
            Some(name_width) => {
                width += name_width + usize::from(index > 0);
            }
            None => return false,
        }
    }

    // the `;`
    width < 80
}
