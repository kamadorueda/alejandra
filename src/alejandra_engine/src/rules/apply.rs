pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let apply = crate::parsers::apply::parse(build_ctx, node);

    let vertical =
        build_ctx.vertical || apply.newline || !apply.comments.is_empty();

    // left
    let element = apply.left.unwrap();
    if vertical {
        steps.push_back(crate::builder::Step::FormatWider(element));
    } else {
        steps.push_back(crate::builder::Step::Format(element));
    }

    // /**/
    let comments = !apply.comments.is_empty();
    if comments {
        for text in apply.comments {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::Comment(text));
        }
    }

    // right
    let element = apply.right.unwrap();
    if vertical {
        if !apply.newline
            && !comments
            && matches!(
                element.kind(),
                rnix::SyntaxKind::NODE_ATTR_SET
                    | rnix::SyntaxKind::NODE_LIST
                    | rnix::SyntaxKind::NODE_PAREN
                    | rnix::SyntaxKind::NODE_STRING
            )
        {
            steps.push_back(crate::builder::Step::Whitespace);
        } else {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        };
        steps.push_back(crate::builder::Step::FormatWider(element));
    } else {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Format(element));
    }

    steps
}
