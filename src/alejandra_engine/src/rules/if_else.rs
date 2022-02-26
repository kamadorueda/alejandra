pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let if_else = crate::parsers::if_else::parse(build_ctx, node);

    // if
    steps.push_back(crate::builder::Step::Token(
        rnix::SyntaxKind::TOKEN_IF,
        "if".to_string(),
    ));

    if if_else.comments_before_if_expr.is_empty() {
        // expr
        let element = if_else.if_expr.unwrap();
        if crate::builder::fits_in_single_line(build_ctx, element.clone()) {
            steps.push_back(crate::builder::Step::Whitespace);
            steps.push_back(crate::builder::Step::FormatWider(element));
        } else {
            steps.push_back(crate::builder::Step::Indent);
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::FormatWider(element));
            steps.push_back(crate::builder::Step::Dedent);
        }
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else {
        // /**/
        steps.push_back(crate::builder::Step::Indent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        for text in if_else.comments_before_if_expr {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        // expr
        steps.push_back(crate::builder::Step::FormatWider(
            if_else.if_expr.unwrap(),
        ));
        steps.push_back(crate::builder::Step::Dedent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    // /**/
    if !if_else.comments_after_if_expr.is_empty() {
        for text in if_else.comments_after_if_expr {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
    }

    // then
    steps.push_back(crate::builder::Step::Token(
        rnix::SyntaxKind::TOKEN_THEN,
        "then".to_string(),
    ));

    if if_else.comments_before_then_expr.is_empty() {
        // expr
        let element = if_else.then_expr.unwrap();
        if matches!(
            element.kind(),
            rnix::SyntaxKind::NODE_ATTR_SET
                | rnix::SyntaxKind::NODE_LET_IN
                | rnix::SyntaxKind::NODE_LIST
                | rnix::SyntaxKind::NODE_STRING
        ) || crate::builder::fits_in_single_line(build_ctx, element.clone())
        {
            steps.push_back(crate::builder::Step::Whitespace);
            steps.push_back(crate::builder::Step::FormatWider(element));
        } else {
            steps.push_back(crate::builder::Step::Indent);
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::FormatWider(element));
            steps.push_back(crate::builder::Step::Dedent);
        }
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else {
        // /**/
        steps.push_back(crate::builder::Step::Indent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        for text in if_else.comments_before_then_expr {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        // expr
        steps.push_back(crate::builder::Step::FormatWider(
            if_else.then_expr.unwrap(),
        ));
        steps.push_back(crate::builder::Step::Dedent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    // /**/
    if !if_else.comments_after_then_expr.is_empty() {
        for text in if_else.comments_after_then_expr {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
    }

    // else
    steps.push_back(crate::builder::Step::Token(
        rnix::SyntaxKind::TOKEN_ELSE,
        "else".to_string(),
    ));

    if if_else.comments_before_else_expr.is_empty() {
        // expr
        let element = if_else.else_expr.unwrap();
        if matches!(
            element.kind(),
            rnix::SyntaxKind::NODE_ATTR_SET
                | rnix::SyntaxKind::NODE_IF_ELSE
                | rnix::SyntaxKind::NODE_LET_IN
                | rnix::SyntaxKind::NODE_LIST
                | rnix::SyntaxKind::NODE_STRING
        ) || crate::builder::fits_in_single_line(build_ctx, element.clone())
        {
            steps.push_back(crate::builder::Step::Whitespace);
            steps.push_back(crate::builder::Step::FormatWider(element));
        } else {
            steps.push_back(crate::builder::Step::Indent);
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::FormatWider(element));
            steps.push_back(crate::builder::Step::Dedent);
        }
    } else {
        // /**/
        steps.push_back(crate::builder::Step::Indent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        for text in if_else.comments_before_else_expr {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        // expr
        steps.push_back(crate::builder::Step::FormatWider(
            if_else.else_expr.unwrap(),
        ));
        steps.push_back(crate::builder::Step::Dedent);
    }

    steps
}
