pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    let mut steps = Vec::new();

    let mut children = crate::annotated_children::annotated(build_ctx, node);

    let first = children.next().unwrap();
    let second = children.next().unwrap();
    let third = children.next().unwrap();

    let vertical = build_ctx.vertical
        || first.has_inline_comment
        || first.has_trivialities
        || second.has_inline_comment
        || second.has_trivialities
        || third.has_inline_comment
        || third.has_trivialities;

    // first
    if vertical {
        let kind = first.element.kind();

        if matches!(kind, rnix::SyntaxKind::NODE_BIN_OP) {
            steps.push(crate::builder::Step::Format(first.element));
        } else {
            steps.push(crate::builder::Step::FormatWider(first.element));
        }
    } else {
        steps.push(crate::builder::Step::Format(first.element));
    }

    if let Some(text) = first.inline_comment {
        steps.push(crate::builder::Step::Whitespace);
        steps.push(crate::builder::Step::Comment(text));
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
    } else if vertical {
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
    }

    for trivia in first.trivialities {
        match trivia {
            crate::annotated_children::Trivia::Comment(text) => {
                steps.push(crate::builder::Step::Comment(text));
                steps.push(crate::builder::Step::NewLine);
                steps.push(crate::builder::Step::Pad);
            }
            crate::annotated_children::Trivia::Newlines => {}
        }
    }

    // second
    if !vertical {
        steps.push(crate::builder::Step::Whitespace);
    }
    steps.push(crate::builder::Step::Format(second.element));

    if let Some(text) = second.inline_comment {
        steps.push(crate::builder::Step::Whitespace);
        steps.push(crate::builder::Step::Comment(text));
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
    }

    if second.has_comments {
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
        for trivia in second.trivialities {
            match trivia {
                crate::annotated_children::Trivia::Comment(text) => {
                    steps.push(crate::builder::Step::Comment(text));
                    steps.push(crate::builder::Step::NewLine);
                    steps.push(crate::builder::Step::Pad);
                }
                crate::annotated_children::Trivia::Newlines => {}
            }
        }
    } else if !second.has_inline_comment {
        steps.push(crate::builder::Step::Whitespace);
    }

    // third
    if vertical {
        steps.push(crate::builder::Step::FormatWider(third.element));
    } else {
        steps.push(crate::builder::Step::Format(third.element));
    }

    steps
}
