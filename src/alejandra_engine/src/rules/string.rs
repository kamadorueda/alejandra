const PLACEHOLDER: &str = "\
    4d13159079d76c1398db5f3ab0c62325\
    f884b545e63226f7ec8aad96c52e13e8\
    6b219abc9462c41b87e47344752e9940\
    abf9353565f69a5db5c672b89372b84c";

pub(crate) fn rule(
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
            if build_ctx.vertical {
                steps.push_back(crate::builder::Step::FormatWider(
                    child.element,
                ));
            } else {
                steps.push_back(crate::builder::Step::Format(child.element));
            }
        }
    } else {
        let elements: Vec<rnix::SyntaxElement> = children
            .get_remaining()
            .iter()
            .map(|child| child.element.clone())
            .collect();

        let mut interpolations = elements
            .iter()
            .filter(|e| e.kind() != rnix::SyntaxKind::TOKEN_STRING_CONTENT);

        let content: String = elements[0..elements.len() - 1]
            .iter()
            .map(|element| match element.kind() {
                rnix::SyntaxKind::TOKEN_STRING_CONTENT => {
                    let token = element.clone().into_token().unwrap();
                    token.text().to_string()
                }
                _ => PLACEHOLDER.to_string(),
            })
            .collect();

        let lines: Vec<String> =
            content.split('\n').map(|line| line.to_string()).collect();

        let should_trim_end: bool =
            !lines.is_empty() && lines[lines.len() - 1].trim().is_empty();

        let mut lines: Vec<String> = lines
            .iter()
            .map(|line| {
                if should_trim_end {
                    line.trim_end().to_string()
                } else {
                    line.to_string()
                }
            })
            .collect();

        let mut indentation: usize = usize::MAX;
        for line in lines.iter() {
            let line = line.trim_end();

            if !line.is_empty() {
                indentation = usize::min(
                    indentation,
                    line.chars().count() - line.trim_start().chars().count(),
                );
            }
        }
        if indentation == usize::MAX {
            indentation = 0;
        };

        // Dedent everything as much as possible
        lines = lines
            .iter()
            .map(|line| {
                if indentation < line.chars().count() {
                    line.chars().skip(indentation).collect::<String>()
                } else {
                    line.to_string()
                }
            })
            .collect();

        // Indent everything 2 spaces
        if lines.len() > 1
            && lines.iter().filter(|line| !line.trim().is_empty()).count() >= 1
        {
            lines = lines
                .iter()
                .map(|line| {
                    if !line.trim().is_empty() {
                        format!("  {}", line)
                    } else {
                        line.to_string()
                    }
                })
                .collect();
        }

        for (index, line) in lines.iter().enumerate() {
            let portions: Vec<String> = line
                .split(PLACEHOLDER)
                .map(|portion| portion.to_string())
                .collect();

            if portions.len() == 1 {
                if !portions[0].is_empty() || index + 1 == lines.len() {
                    if lines.len() > 1 {
                        steps.push_back(crate::builder::Step::Pad);
                    }
                    steps.push_back(crate::builder::Step::Token(
                        rnix::SyntaxKind::TOKEN_STRING_CONTENT,
                        portions[0].to_string(),
                    ));
                }
            } else {
                if lines.len() > 1 {
                    steps.push_back(crate::builder::Step::Pad);
                }
                for (index, portion) in portions.iter().enumerate() {
                    steps.push_back(crate::builder::Step::Token(
                        rnix::SyntaxKind::TOKEN_STRING_CONTENT,
                        portion.to_string(),
                    ));

                    if index + 1 != portions.len() {
                        steps.push_back(crate::builder::Step::Indent);
                        steps.push_back(crate::builder::Step::FormatWider(
                            interpolations.next().unwrap().clone(),
                        ));
                        steps.push_back(crate::builder::Step::Dedent);
                    }
                }
            }

            if index + 1 < lines.len() {
                steps.push_back(crate::builder::Step::NewLine);
            }
        }

        for interpolation in interpolations {
            steps.push_back(crate::builder::Step::FormatWider(
                interpolation.clone(),
            ));
        }
    }

    steps
}
