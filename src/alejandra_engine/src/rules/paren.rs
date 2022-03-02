pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children2::new(build_ctx, node);

    let opener = children.next().unwrap();
    let expression = children.next().unwrap();
    let closer = children.next().unwrap();

    let vertical = opener.has_inline_comment
        || opener.has_comments
        || expression.has_inline_comment
        || expression.has_comments
        || closer.has_inline_comment
        || closer.has_comments
        || matches!(
            expression.element.kind(),
            rnix::SyntaxKind::NODE_IF_ELSE | rnix::SyntaxKind::NODE_LET_IN
        )
        || (matches!(
            expression.element.kind(),
            rnix::SyntaxKind::NODE_APPLY
                | rnix::SyntaxKind::NODE_ASSERT
                | rnix::SyntaxKind::NODE_BIN_OP
                | rnix::SyntaxKind::NODE_OR_DEFAULT
                | rnix::SyntaxKind::NODE_LAMBDA
                | rnix::SyntaxKind::NODE_SELECT
                | rnix::SyntaxKind::NODE_WITH
        ) && second_through_penultimate_line_are_not_indented(
            build_ctx,
            expression.element.clone(),
        ));

    // opener
    steps.push_back(crate::builder::Step::Format(opener.element));
    if vertical {
        steps.push_back(crate::builder::Step::Indent);
    }

    if let Some(text) = opener.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else if vertical {
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
    if vertical {
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
    if vertical {
        steps.push_back(crate::builder::Step::Dedent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }
    steps.push_back(crate::builder::Step::Format(closer.element));

    steps
}

fn second_through_penultimate_line_are_not_indented(
    build_ctx: &crate::builder::BuildCtx,
    element: rnix::SyntaxElement,
) -> bool {
    let mut build_ctx =
        crate::builder::BuildCtx { force_wide: false, ..build_ctx.clone() };

    let formatted =
        crate::builder::build(&mut build_ctx, element).unwrap().to_string();

    let formatted_lines: Vec<&str> = formatted.split('\n').collect();

    if formatted_lines.len() <= 2 {
        return false;
    }

    let whitespace = format!("{0:<1$}", "", 2 * (build_ctx.indentation + 1));

    formatted_lines
        .iter()
        .skip(1)
        .rev()
        .skip(1)
        .any(|line| !line.is_empty() && !line.starts_with(&whitespace))
}
