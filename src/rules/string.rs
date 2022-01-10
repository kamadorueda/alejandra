pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new(build_ctx, node);

    let child = children.get_next().unwrap();
    let child_token = child.element.clone().into_token().unwrap();
    let text = child_token.text();
    steps.push_back(crate::builder::Step::Format(child.element));

    if text == "\"" {
        while let Some(child) = children.get_next() {
            match build_ctx.config.layout() {
                crate::config::Layout::Tall => {
                    steps.push_back(crate::builder::Step::FormatWider(
                        child.element,
                    ));
                }
                crate::config::Layout::Wide => {
                    steps
                        .push_back(crate::builder::Step::Format(child.element));
                }
            }
        }
    } else {
        let indentation = get_double_quoted_string_indentation(&node);

        while let Some(child) = children.peek_next() {
            match child.element.kind() {
                rnix::SyntaxKind::TOKEN_STRING_CONTENT => {
                    let child_token = child.element.into_token().unwrap();
                    let lines: Vec<&str> =
                        child_token.text().split('\n').collect();

                    children.move_next();
                    for (index, line) in lines.iter().enumerate() {
                        if index + 1 == lines.len() && line.trim().len() == 0 {
                            if let rnix::SyntaxKind::TOKEN_STRING_END =
                                children.peek_next().unwrap().element.kind()
                            {
                                continue;
                            }
                        }

                        steps.push_back(crate::builder::Step::Token(
                            rnix::SyntaxKind::TOKEN_STRING_CONTENT,
                            if indentation >= line.len() {
                                line.to_string()
                            } else {
                                line[indentation..line.len()].to_string()
                            },
                        ));

                        if index == 0 && lines.len() > 1 {
                            steps.push_back(crate::builder::Step::NewLine);
                            steps.push_back(crate::builder::Step::Pad);
                        } else if index + 1 < lines.len()
                            && lines[index + 1].trim().len() == 0
                        {
                            steps.push_back(crate::builder::Step::NewLine);
                            steps.push_back(crate::builder::Step::Pad);
                        }
                    }
                }
                rnix::SyntaxKind::TOKEN_STRING_END => {
                    steps
                        .push_back(crate::builder::Step::Format(child.element));
                    children.move_next();
                }
                _ => {
                    steps.push_back(crate::builder::Step::FormatWider(
                        child.element,
                    ));
                    children.move_next();
                }
            }
        }
    }

    // steps = crate::rules::default(build_ctx, node);
    steps
}

fn get_double_quoted_string_indentation(node: &rnix::SyntaxNode) -> usize {
    let mut indentation: usize = usize::MAX;

    let text: String = node
        .children_with_tokens()
        .filter(|child| child.kind() == rnix::SyntaxKind::TOKEN_STRING_CONTENT)
        .map(|child| child.into_token().unwrap())
        .map(|token| token.text().to_string())
        .collect();

    for line in text.split('\n') {
        let line = line.trim_end();

        if line.len() > 0 {
            indentation =
                usize::min(indentation, line.len() - line.trim_start().len());
        }
    }

    if indentation == usize::MAX { 0 } else { indentation }
}
