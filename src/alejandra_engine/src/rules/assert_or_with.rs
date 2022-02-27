pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let parsed =
        crate::parsers::assert_or_with::AssertOrWith::new(build_ctx, node);

    let vertical = build_ctx.vertical
        || !parsed.comments_after_assert_or_with.is_empty()
        || parsed.has_newlines_after_assert_or_with
        || !parsed.comments_after_semicolon.is_empty()
        || parsed.has_newlines_after_semicolon;

    // assert_or_with
    steps.push_back(crate::builder::Step::Format(parsed.assert_or_with));

    // comments_after_assert_or_with
    if parsed.comments_after_assert_or_with.is_empty() {
        steps.push_back(crate::builder::Step::Whitespace);
    } else {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        for text in parsed.comments_after_assert_or_with {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
    }

    // first_expression
    if vertical {
        steps.push_back(crate::builder::Step::FormatWider(
            parsed.first_expression,
        ));
    } else {
        steps.push_back(crate::builder::Step::Format(parsed.first_expression));
    }

    // semicolon
    steps.push_back(crate::builder::Step::Format(parsed.semicolon));

    // comments_after_semicolon
    let has_comments_after_semicolon =
        !parsed.comments_after_semicolon.is_empty();
    for text in parsed.comments_after_semicolon {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        steps.push_back(crate::builder::Step::Comment(text));
    }

    // second_expression
    if vertical {
        if matches!(
            parsed.second_expression.kind(),
            rnix::SyntaxKind::NODE_ASSERT | rnix::SyntaxKind::NODE_WITH
        ) {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::FormatWider(
                parsed.second_expression,
            ));
        } else if has_comments_after_semicolon
            || !matches!(
                parsed.second_expression.kind(),
                rnix::SyntaxKind::NODE_ATTR_SET
                    | rnix::SyntaxKind::NODE_IDENT
                    | rnix::SyntaxKind::NODE_PAREN
                    | rnix::SyntaxKind::NODE_LET_IN
                    | rnix::SyntaxKind::NODE_LIST
                    | rnix::SyntaxKind::NODE_LITERAL
                    | rnix::SyntaxKind::NODE_STRING
            )
        {
            steps.push_back(crate::builder::Step::Indent);
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::FormatWider(
                parsed.second_expression,
            ));
            steps.push_back(crate::builder::Step::Dedent);
        } else {
            steps.push_back(crate::builder::Step::Whitespace);
            steps.push_back(crate::builder::Step::FormatWider(
                parsed.second_expression,
            ));
        }
    } else {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Format(parsed.second_expression));
    }

    steps
}
