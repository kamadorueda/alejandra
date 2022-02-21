pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new(build_ctx, node);

    for branch in ["if", "then", "else"] {
        // if/then/else
        let child = children.get_next().unwrap();
        steps.push_back(crate::builder::Step::Format(child.element));

        if let rnix::SyntaxKind::TOKEN_COMMENT =
            children.peek_next().unwrap().element.kind()
        {
            steps.push_back(crate::builder::Step::Indent);

            // /**/
            children.drain_comments(|text| {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::Comment(text));
            });

            // expr
            let child = children.get_next().unwrap();
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::FormatWider(child.element));
            steps.push_back(crate::builder::Step::Dedent);
        } else {
            let child = children.get_next().unwrap();

            // expr
            if crate::builder::fits_in_single_line(
                build_ctx,
                child.element.clone(),
            ) || (branch == "else"
                && child.element.kind() == rnix::SyntaxKind::NODE_IF_ELSE)
            {
                steps.push_back(crate::builder::Step::Whitespace);
                steps.push_back(crate::builder::Step::FormatWider(
                    child.element,
                ));
            } else {
                steps.push_back(crate::builder::Step::Indent);
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::FormatWider(
                    child.element,
                ));
                steps.push_back(crate::builder::Step::Dedent);
            }
        }

        if branch != "else" {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);

            // /**/
            children.drain_comments(|text| {
                steps.push_back(crate::builder::Step::Comment(text));
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            });
        }
    }

    steps
}
