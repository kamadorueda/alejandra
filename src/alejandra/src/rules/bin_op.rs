pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    rule_with_configuration(build_ctx, node, "bin_op_and_or_default")
}

pub(crate) fn rule_with_configuration(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
    parent_kind: &str,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children2::new(build_ctx, node);

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

        if (parent_kind == "bin_op_and_or_default"
            && matches!(
                kind,
                rnix::SyntaxKind::NODE_BIN_OP
                    | rnix::SyntaxKind::NODE_OR_DEFAULT
            ))
            || (parent_kind == "select"
                && matches!(kind, rnix::SyntaxKind::NODE_SELECT))
        {
            steps.push_back(crate::builder::Step::Format(first.element));
        } else {
            steps.push_back(crate::builder::Step::FormatWider(first.element));
        }
    } else {
        steps.push_back(crate::builder::Step::Format(first.element));
    }

    if let Some(text) = first.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else if vertical {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    for trivia in first.trivialities {
        match trivia {
            crate::children2::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::Comment(text));
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
            crate::children2::Trivia::Newlines => {}
        }
    }

    // second
    if !vertical && parent_kind == "bin_op_and_or_default" {
        steps.push_back(crate::builder::Step::Whitespace);
    }
    steps.push_back(crate::builder::Step::Format(second.element));

    if let Some(text) = second.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    if second.has_comments {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        for trivia in second.trivialities {
            match trivia {
                crate::children2::Trivia::Comment(text) => {
                    steps.push_back(crate::builder::Step::Comment(text));
                    steps.push_back(crate::builder::Step::NewLine);
                    steps.push_back(crate::builder::Step::Pad);
                }
                crate::children2::Trivia::Newlines => {}
            }
        }
    } else if !second.has_inline_comment
        && parent_kind == "bin_op_and_or_default"
    {
        steps.push_back(crate::builder::Step::Whitespace);
    }

    // third
    if vertical {
        steps.push_back(crate::builder::Step::FormatWider(third.element));
    } else {
        steps.push_back(crate::builder::Step::Format(third.element));
    }

    steps
}
