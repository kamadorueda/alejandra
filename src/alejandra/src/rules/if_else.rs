pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children2::new(build_ctx, node);

    let if_ = children.next().unwrap();
    let if_expr = children.next().unwrap();
    let then_ = children.next().unwrap();
    let then_expr = children.next().unwrap();
    let else_ = children.next().unwrap();
    let else_expr = children.next().unwrap();

    // if_
    steps.push_back(crate::builder::Step::Format(if_.element));

    steps.push_back(crate::builder::Step::Indent);
    if let Some(text) = if_.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
    }

    for trivia in if_.trivialities {
        match trivia {
            crate::children2::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::Comment(text));
            }
            crate::children2::Trivia::Newlines => {}
        }
    }
    steps.push_back(crate::builder::Step::Dedent);

    // if_expr
    if !if_.has_inline_comment
        && !if_.has_comments
        && crate::builder::fits_in_single_line(
            build_ctx,
            if_expr.element.clone(),
        )
    {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::FormatWider(if_expr.element));
    } else {
        steps.push_back(crate::builder::Step::Indent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        steps.push_back(crate::builder::Step::FormatWider(if_expr.element));
        steps.push_back(crate::builder::Step::Dedent);
    }

    if let Some(text) = if_expr.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    for trivia in if_expr.trivialities {
        match trivia {
            crate::children2::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::Comment(text));
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
            crate::children2::Trivia::Newlines => {}
        }
    }

    // then_
    steps.push_back(crate::builder::Step::Format(then_.element));

    steps.push_back(crate::builder::Step::Indent);
    if let Some(text) = then_.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
    }

    for trivia in then_.trivialities {
        match trivia {
            crate::children2::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::Comment(text));
            }
            crate::children2::Trivia::Newlines => {}
        }
    }
    steps.push_back(crate::builder::Step::Dedent);

    // then_expr
    if !then_.has_inline_comment
        && !then_.has_comments
        && (matches!(
            then_expr.element.kind(),
            rnix::SyntaxKind::NODE_ATTR_SET
                | rnix::SyntaxKind::NODE_LET_IN
                | rnix::SyntaxKind::NODE_LIST
                | rnix::SyntaxKind::NODE_STRING
        ) || crate::builder::fits_in_single_line(
            build_ctx,
            then_expr.element.clone(),
        ))
    {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::FormatWider(then_expr.element));
    } else {
        steps.push_back(crate::builder::Step::Indent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        steps.push_back(crate::builder::Step::FormatWider(then_expr.element));
        steps.push_back(crate::builder::Step::Dedent);
    }

    if let Some(text) = then_expr.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    for trivia in then_expr.trivialities {
        match trivia {
            crate::children2::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::Comment(text));
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
            crate::children2::Trivia::Newlines => {}
        }
    }

    // else_
    steps.push_back(crate::builder::Step::Format(else_.element));

    steps.push_back(crate::builder::Step::Indent);
    if let Some(text) = else_.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
    }

    for trivia in else_.trivialities {
        match trivia {
            crate::children2::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::Comment(text));
            }
            crate::children2::Trivia::Newlines => {}
        }
    }
    steps.push_back(crate::builder::Step::Dedent);

    // else_expr
    if !else_.has_inline_comment
        && !else_.has_comments
        && (matches!(
            else_expr.element.kind(),
            rnix::SyntaxKind::NODE_ATTR_SET
                | rnix::SyntaxKind::NODE_IF_ELSE
                | rnix::SyntaxKind::NODE_LET_IN
                | rnix::SyntaxKind::NODE_LIST
                | rnix::SyntaxKind::NODE_STRING
        ) || crate::builder::fits_in_single_line(
            build_ctx,
            else_expr.element.clone(),
        ))
    {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::FormatWider(else_expr.element));
    } else {
        steps.push_back(crate::builder::Step::Indent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        steps.push_back(crate::builder::Step::FormatWider(else_expr.element));
        steps.push_back(crate::builder::Step::Dedent);
    }

    steps
}
