use crate::children2::Child;

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

    let mut children: Vec<Child> =
        crate::children2::new(build_ctx, node).collect();

    children.iter().for_each(|x| {
        println!("{:?}", x.element);
    });

    let vertical = build_ctx.vertical
        || children
            .iter()
            .any(|child| child.has_inline_comment || child.has_trivialities);

    // first
    if vertical {
        let kind = first.element.kind();

        if (parent_kind == "bin_op_and_or_default"
            && matches!(kind, rnix::SyntaxKind::NODE_BIN_OP))
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
