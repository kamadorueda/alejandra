pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let parsed = crate::parsers::if_else::IfElse::parse(build_ctx, node);

    // if_
    steps.push_back(crate::builder::Step::Format(parsed.if_));

    if parsed.comments_before_if_expr.is_empty() {
        // if_expr
        if crate::builder::fits_in_single_line(
            build_ctx,
            parsed.if_expr.clone(),
        ) {
            steps.push_back(crate::builder::Step::Whitespace);
            steps.push_back(crate::builder::Step::FormatWider(parsed.if_expr));
        } else {
            steps.push_back(crate::builder::Step::Indent);
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::FormatWider(parsed.if_expr));
            steps.push_back(crate::builder::Step::Dedent);
        }
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else {
        // comments_before_if_expr
        steps.push_back(crate::builder::Step::Indent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        for text in parsed.comments_before_if_expr {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        // if_expr
        steps.push_back(crate::builder::Step::FormatWider(parsed.if_expr));
        steps.push_back(crate::builder::Step::Dedent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    // comments_after_if_expr
    for text in parsed.comments_after_if_expr {
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    // then_
    steps.push_back(crate::builder::Step::Format(parsed.then_));

    if parsed.comments_before_then_expr.is_empty() {
        // then_expr
        if matches!(
            parsed.then_expr.kind(),
            rnix::SyntaxKind::NODE_ATTR_SET
                | rnix::SyntaxKind::NODE_LET_IN
                | rnix::SyntaxKind::NODE_LIST
                | rnix::SyntaxKind::NODE_STRING
        ) || crate::builder::fits_in_single_line(
            build_ctx,
            parsed.then_expr.clone(),
        ) {
            steps.push_back(crate::builder::Step::Whitespace);
            steps
                .push_back(crate::builder::Step::FormatWider(parsed.then_expr));
        } else {
            steps.push_back(crate::builder::Step::Indent);
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps
                .push_back(crate::builder::Step::FormatWider(parsed.then_expr));
            steps.push_back(crate::builder::Step::Dedent);
        }
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else {
        // comments_before_then_expr
        steps.push_back(crate::builder::Step::Indent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        for text in parsed.comments_before_then_expr {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        // then_expr
        steps.push_back(crate::builder::Step::FormatWider(parsed.then_expr));
        steps.push_back(crate::builder::Step::Dedent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    // comments_after_then_expr
    for text in parsed.comments_after_then_expr {
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    // else_
    steps.push_back(crate::builder::Step::Format(parsed.else_));

    if parsed.comments_before_else_expr.is_empty() {
        // else_expr
        if matches!(
            parsed.else_expr.kind(),
            rnix::SyntaxKind::NODE_ATTR_SET
                | rnix::SyntaxKind::NODE_IF_ELSE
                | rnix::SyntaxKind::NODE_LET_IN
                | rnix::SyntaxKind::NODE_LIST
                | rnix::SyntaxKind::NODE_STRING
        ) || crate::builder::fits_in_single_line(
            build_ctx,
            parsed.else_expr.clone(),
        ) {
            steps.push_back(crate::builder::Step::Whitespace);
            steps
                .push_back(crate::builder::Step::FormatWider(parsed.else_expr));
        } else {
            steps.push_back(crate::builder::Step::Indent);
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps
                .push_back(crate::builder::Step::FormatWider(parsed.else_expr));
            steps.push_back(crate::builder::Step::Dedent);
        }
    } else {
        // comments_before_else_expr
        steps.push_back(crate::builder::Step::Indent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        for text in parsed.comments_before_else_expr {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        // else_expr
        steps.push_back(crate::builder::Step::FormatWider(parsed.else_expr));
        steps.push_back(crate::builder::Step::Dedent);
    }

    steps
}
