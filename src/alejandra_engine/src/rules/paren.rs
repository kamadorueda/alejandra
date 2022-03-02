pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children2::new(build_ctx, node);

    let opener = children.next().unwrap();
    let expression = children.next().unwrap();
    let closer = children.next().unwrap();

    let loose = opener.has_inline_comment
        || opener.has_comments
        || expression.has_inline_comment
        || expression.has_comments
        || closer.has_inline_comment
        || closer.has_comments
        || matches!(expression.element.kind(), rnix::SyntaxKind::NODE_IF_ELSE);

    let should_indent = loose
        || matches!(
            expression.element.kind(),
            rnix::SyntaxKind::NODE_APPLY
                | rnix::SyntaxKind::NODE_ASSERT
                | rnix::SyntaxKind::NODE_BIN_OP
                | rnix::SyntaxKind::NODE_OR_DEFAULT
                | rnix::SyntaxKind::NODE_LAMBDA
                | rnix::SyntaxKind::NODE_SELECT
                | rnix::SyntaxKind::NODE_WITH
        ) && !crate::utils::second_through_penultimate_line_are_indented(
            build_ctx,
            expression.element.clone(),
            matches!(expression.element.kind(), rnix::SyntaxKind::NODE_LAMBDA),
        );

    // opener
    steps.push_back(crate::builder::Step::Format(opener.element));
    if should_indent {
        steps.push_back(crate::builder::Step::Indent);
    }

    if let Some(text) = opener.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else if loose {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    for trivia in opener.trivialities {
        match trivia {
            crate::children2::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::Comment(text));
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
            crate::children2::Trivia::Newlines(_) => {}
        }
    }

    // expression
    if loose {
        steps.push_back(crate::builder::Step::FormatWider(expression.element));
    } else {
        steps.push_back(crate::builder::Step::Format(expression.element));
    }

    if let Some(text) = expression.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
    }

    for trivia in expression.trivialities {
        match trivia {
            crate::children2::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::Comment(text));
            }
            crate::children2::Trivia::Newlines(_) => {}
        }
    }

    // closer
    if should_indent {
        steps.push_back(crate::builder::Step::Dedent);
    }

    if loose {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }
    steps.push_back(crate::builder::Step::Format(closer.element));

    steps
}
