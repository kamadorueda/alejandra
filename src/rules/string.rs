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
        let placeholder = get_placeholder();

        let elements: Vec<rnix::SyntaxElement> = children
            .get_remaining()
            .iter()
            .map(|child| child.element.clone())
            .collect();

        let mut interpolations = elements
            .iter()
            .filter(|e| e.kind() != rnix::SyntaxKind::TOKEN_STRING_CONTENT);

        let mut lines: Vec<String> = elements[0..elements.len() - 1]
            .iter()
            .map(|element| match element.kind() {
                rnix::SyntaxKind::TOKEN_STRING_CONTENT => {
                    let token = element.clone().into_token().unwrap();
                    token.text().to_string()
                }
                _ => placeholder.to_string(),
            })
            .collect::<String>()
            .split('\n')
            .map(|line| line.trim_end().to_string())
            .collect();

        // eprintln!("0: {:?}", lines);

        let mut indentation: usize = usize::MAX;
        for line in lines.iter() {
            let line = line.trim_end();

            if line.len() > 0 {
                indentation = usize::min(
                    indentation,
                    line.len() - line.trim_start().len(),
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
                if indentation < line.len() {
                    line[indentation..line.len()].to_string()
                } else {
                    line.to_string()
                }
            })
            .collect();

        // eprintln!("1: ''{}''", lines.join("\n"));
        // eprintln!("indentation={}, placeholder={}", indentation, placeholder);

        for (index, line) in lines.iter().enumerate() {
            let portions: Vec<String> = line
                .split(&placeholder)
                .map(|portion| portion.to_string())
                .collect();

            if portions.len() == 1 {
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::Token(
                    rnix::SyntaxKind::TOKEN_STRING_CONTENT,
                    portions[0].to_string(),
                ));
            } else {
                steps.push_back(crate::builder::Step::Pad);
                for (index, portion) in portions.iter().enumerate() {
                    steps.push_back(crate::builder::Step::Token(
                        rnix::SyntaxKind::TOKEN_STRING_CONTENT,
                        portion.to_string(),
                    ));

                    if index + 1 != portions.len() {
                        steps.push_back(crate::builder::Step::FormatWider(
                            interpolations.next().unwrap().clone(),
                        ));
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

fn get_placeholder() -> String {
    use rand::RngCore;

    let mut bytes = [0u8; 32];

    rand::thread_rng().fill_bytes(&mut bytes);

    bytes.iter().map(|byte| format!("{:02X}", byte)).collect()
}
