pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children2::new(build_ctx, node);

    let first = children.next().unwrap();
    let second = children.next().unwrap();
    let third = children.next().unwrap();
    let fourth = children.next().unwrap();

    let vertical = build_ctx.vertical
        || first.has_inline_comment
        || first.has_trivialities
        || second.has_inline_comment
        || second.has_trivialities
        || third.has_inline_comment
        || third.has_trivialities
        || fourth.has_inline_comment
        || fourth.has_trivialities;

    // first
    steps.push_back(crate::builder::Step::Format(first.element));

    if let Some(text) = first.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else if first.has_comments {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else {
        steps.push_back(crate::builder::Step::Whitespace);
    }

    for trivia in first.trivialities {
        match trivia {
            crate::children2::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::Comment(text));
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
            crate::children2::Trivia::Newlines(_) => {}
        }
    }

    // second
    if vertical {
        steps.push_back(crate::builder::Step::FormatWider(second.element));
    } else {
        steps.push_back(crate::builder::Step::Format(second.element));
    }

    // third
    steps.push_back(crate::builder::Step::Format(third.element));

    if let Some(text) = third.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    for trivia in third.trivialities {
        match trivia {
            crate::children2::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::Comment(text));
            }
            crate::children2::Trivia::Newlines(_) => {}
        }
    }

    // fourth
    if vertical {
        if matches!(
            fourth.element.kind(),
            rnix::SyntaxKind::NODE_ASSERT | rnix::SyntaxKind::NODE_WITH
        ) {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::FormatWider(fourth.element));
        } else if third.has_inline_comment
            || third.has_comments
            || !matches!(
                fourth.element.kind(),
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
            steps.push_back(crate::builder::Step::FormatWider(fourth.element));
            steps.push_back(crate::builder::Step::Dedent);
        } else {
            steps.push_back(crate::builder::Step::Whitespace);
            steps.push_back(crate::builder::Step::FormatWider(fourth.element));
        }
    } else {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Format(fourth.element));
    }

    steps
}
