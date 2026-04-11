use crate::config::Indentation;

const PLACEHOLDER: &str = "\
    4d13159079d76c1398db5f3ab0c62325\
    f884b545e63226f7ec8aad96c52e13e8\
    6b219abc9462c41b87e47344752e9940\
    abf9353565f69a5db5c672b89372b84c";

/// Returns the indentation unit to use inside `''..''` string content.
///
/// Nix only strips leading **spaces** when evaluating indented strings.
/// We must never use tabs in string content, even when the surrounding code
/// uses tab indentation. This ensures string values are preserved correctly.
fn string_content_indent_unit(indentation: Indentation) -> &'static str {
    match indentation {
        Indentation::FourSpaces => "    ",
        Indentation::Tabs | Indentation::TwoSpaces => "  ",
    }
}

pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    let mut steps = Vec::new();

    let mut children = crate::children::Children::new(build_ctx, node);

    let child = children.get_next().unwrap();
    let child_token = child.clone().into_token().unwrap();
    let text = child_token.text();
    steps.push(crate::builder::Step::Format(child));

    if text == "\"" {
        while let Some(child) = children.get_next() {
            if build_ctx.vertical {
                steps.push(crate::builder::Step::FormatWider(child));
            } else {
                steps.push(crate::builder::Step::Format(child));
            }
        }
    } else {
        let elements: Vec<rnix::SyntaxElement> =
            children.get_remaining().to_vec();

        let mut interpolations = elements
            .iter()
            .filter(|e| e.kind() != rnix::SyntaxKind::TOKEN_STRING_CONTENT);

        let content: String = elements[0..elements.len() - 1]
            .iter()
            .map(|element| match element.kind() {
                rnix::SyntaxKind::TOKEN_STRING_CONTENT => {
                    element.as_token().unwrap().to_string()
                }
                _ => PLACEHOLDER.to_string(),
            })
            .collect();

        let lines: Vec<&str> = content.split('\n').collect();

        // IMPORTANT: Nix preserves trailing whitespace in multiline strings.
        // We must NOT trim trailing whitespace from content lines — it's semantically significant.
        // However, trim the last line IF it's whitespace-only (it's just formatting, not content).
        let mut lines: Vec<String> = lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                // Only trim the very last line if it's whitespace-only
                if i == lines.len() - 1 && line.trim().is_empty() {
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
                // Only count leading spaces, not tabs. Nix only strips spaces
                // from multiline strings.
                let leading_spaces = line.chars().count()
                    - line.trim_start_matches(' ').chars().count();
                indentation = usize::min(indentation, leading_spaces);
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

        // Indent everything
        if lines.len() > 1
            && lines.iter().filter(|line| !line.trim().is_empty()).count() >= 1
        {
            lines = lines
                .iter()
                .map(|line| {
                    if !line.trim().is_empty() {
                        format!(
                            "{}{}",
                            string_content_indent_unit(
                                build_ctx.config.indentation
                            ),
                            line
                        )
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
                        let content_pad = string_content_indent_unit(
                            build_ctx.config.indentation,
                        )
                        .repeat(build_ctx.indentation);
                        if !content_pad.is_empty() {
                            steps.push(crate::builder::Step::Token(
                                rnix::SyntaxKind::TOKEN_WHITESPACE,
                                content_pad,
                            ));
                        }
                    }
                    steps.push(crate::builder::Step::Token(
                        rnix::SyntaxKind::TOKEN_STRING_CONTENT,
                        portions[0].to_string(),
                    ));
                }
            } else {
                if lines.len() > 1 {
                    let content_pad = string_content_indent_unit(
                        build_ctx.config.indentation,
                    )
                    .repeat(build_ctx.indentation);
                    if !content_pad.is_empty() {
                        steps.push(crate::builder::Step::Token(
                            rnix::SyntaxKind::TOKEN_WHITESPACE,
                            content_pad,
                        ));
                    }
                }
                for (index, portion) in portions.iter().enumerate() {
                    steps.push(crate::builder::Step::Token(
                        rnix::SyntaxKind::TOKEN_STRING_CONTENT,
                        portion.to_string(),
                    ));

                    if index + 1 != portions.len() {
                        steps.push(crate::builder::Step::Indent);
                        steps.push(crate::builder::Step::FormatWider(
                            interpolations.next().unwrap().clone(),
                        ));
                        steps.push(crate::builder::Step::Dedent);
                    }
                }
            }

            if index + 1 < lines.len() {
                steps.push(crate::builder::Step::NewLine);
            }
        }

        for interpolation in interpolations {
            steps
                .push(crate::builder::Step::FormatWider(interpolation.clone()));
        }
    }

    steps
}
