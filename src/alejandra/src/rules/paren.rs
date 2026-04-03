pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    let mut steps = Vec::new();

    let mut children = crate::annotated_children::annotated(build_ctx, node);

    let opener = children.next().unwrap();
    let expression = children.next().unwrap();
    let closer = children.next().unwrap();

    let loose = opener.has_inline_comment
        || opener.has_comments
        || expression.has_inline_comment
        || expression.has_comments
        || closer.has_inline_comment
        || closer.has_comments
        || matches!(expression.element.kind(), rnix::SyntaxKind::NODE_IF_ELSE)
        || ((opener.has_trivialities
            || expression.has_trivialities
            || closer.has_trivialities)
            && !matches!(
                expression.element.kind(),
                rnix::SyntaxKind::NODE_ATTR_SET
                    | rnix::SyntaxKind::NODE_IDENT
                    | rnix::SyntaxKind::NODE_LITERAL
                    | rnix::SyntaxKind::NODE_LIST
                    | rnix::SyntaxKind::NODE_STRING
                    | rnix::SyntaxKind::NODE_UNARY_OP
            ));

    let should_indent = loose
        || matches!(
            expression.element.kind(),
            rnix::SyntaxKind::NODE_APPLY
                | rnix::SyntaxKind::NODE_ASSERT
                | rnix::SyntaxKind::NODE_BIN_OP
                | rnix::SyntaxKind::NODE_LAMBDA
                | rnix::SyntaxKind::NODE_SELECT
                | rnix::SyntaxKind::NODE_WITH
        ) && !crate::utils::second_through_penultimate_line_are_indented(
            build_ctx,
            expression.element.clone(),
            matches!(expression.element.kind(), rnix::SyntaxKind::NODE_LAMBDA),
        );

    // opener
    steps.push(crate::builder::Step::Format(opener.element));
    if should_indent {
        steps.push(crate::builder::Step::Indent);
    }

    if opener.inline_comment.is_some() {
        crate::annotated_children::emit_inline_comment(
            &opener.inline_comment,
            &mut steps,
        );
    } else if loose {
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
    }

    crate::annotated_children::emit_trivialities_comment_first(
        &opener.trivialities,
        &mut steps,
    );

    // expression
    if loose {
        steps.push(crate::builder::Step::FormatWider(expression.element));
    } else {
        steps.push(crate::builder::Step::Format(expression.element));
    }

    if let Some(text) = expression.inline_comment {
        steps.push(crate::builder::Step::Whitespace);
        steps.push(crate::builder::Step::Comment(text));
    }

    crate::annotated_children::emit_trivialities_newline_first(
        &expression.trivialities,
        &mut steps,
    );

    // closer
    if should_indent {
        steps.push(crate::builder::Step::Dedent);
    }

    if loose {
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
    }
    steps.push(crate::builder::Step::Format(closer.element));

    steps
}
