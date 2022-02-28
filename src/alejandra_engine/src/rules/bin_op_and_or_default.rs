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

    let parsed = crate::parsers::bin_op::BinOp::parse(build_ctx, node);

    let vertical = build_ctx.vertical
        || !parsed.comments_before_operator.is_empty()
        || parsed.has_newlines_before_operator
        || !parsed.comments_after_operator.is_empty()
        || parsed.has_newlines_after_operator;

    // left_expression
    if vertical {
        let kind = parsed.left_expression.kind();

        if (parent_kind == "bin_op_and_or_default"
            && matches!(
                kind,
                rnix::SyntaxKind::NODE_BIN_OP
                    | rnix::SyntaxKind::NODE_OR_DEFAULT
            ))
            || (parent_kind == "select"
                && matches!(kind, rnix::SyntaxKind::NODE_SELECT))
        {
            steps.push_back(crate::builder::Step::Format(
                parsed.left_expression,
            ));
        } else {
            steps.push_back(crate::builder::Step::FormatWider(
                parsed.left_expression,
            ));
        }
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else {
        steps.push_back(crate::builder::Step::Format(parsed.left_expression));
    }

    // comments_before_operator
    for text in parsed.comments_before_operator {
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    // operator
    if !vertical && parent_kind == "bin_op_and_or_default" {
        steps.push_back(crate::builder::Step::Whitespace);
    }
    steps.push_back(crate::builder::Step::Format(parsed.operator));

    // comments_before_operator
    if parsed.comments_after_operator.is_empty() {
        if parent_kind == "bin_op_and_or_default" {
            steps.push_back(crate::builder::Step::Whitespace);
        }
    } else {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        for text in parsed.comments_after_operator {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
    }

    // right_expression
    if vertical {
        steps.push_back(crate::builder::Step::FormatWider(
            parsed.right_expression,
        ));
    } else {
        steps.push_back(crate::builder::Step::Format(parsed.right_expression));
    }

    steps
}
