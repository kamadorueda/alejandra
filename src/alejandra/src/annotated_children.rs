pub(crate) enum Trivia {
    Comment(String),
    Newlines,
}

pub(crate) struct Child {
    pub element: rnix::SyntaxElement,

    pub inline_comment:     Option<String>,
    pub has_inline_comment: bool,

    pub trivialities:     Vec<Trivia>,
    pub has_comments:     bool,
    pub has_trivialities: bool,
}

/// Emit an inline comment: Whitespace + Comment + NewLine + Pad
pub(crate) fn emit_inline_comment(
    inline_comment: &Option<String>,
    steps: &mut Vec<crate::builder::Step>,
) {
    if let Some(text) = inline_comment {
        steps.push(crate::builder::Step::Whitespace);
        steps.push(crate::builder::Step::Comment(text.clone()));
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
    }
}

/// Emit trivialities with NewLine + Pad + Comment style
pub(crate) fn emit_trivialities_newline_first(
    trivialities: &[Trivia],
    steps: &mut Vec<crate::builder::Step>,
) {
    for trivia in trivialities {
        match trivia {
            Trivia::Comment(text) => {
                steps.push(crate::builder::Step::NewLine);
                steps.push(crate::builder::Step::Pad);
                steps.push(crate::builder::Step::Comment(text.clone()));
            }
            Trivia::Newlines => {}
        }
    }
}

/// Emit trivialities with Comment + NewLine + Pad style
pub(crate) fn emit_trivialities_comment_first(
    trivialities: &[Trivia],
    steps: &mut Vec<crate::builder::Step>,
) {
    for trivia in trivialities {
        match trivia {
            Trivia::Comment(text) => {
                steps.push(crate::builder::Step::Comment(text.clone()));
                steps.push(crate::builder::Step::NewLine);
                steps.push(crate::builder::Step::Pad);
            }
            Trivia::Newlines => {}
        }
    }
}

pub(crate) fn annotated(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::vec::IntoIter<Child> {
    let mut children = crate::children::Children::new(build_ctx, node);

    let mut elements = Vec::new();

    while let Some(element) = children.get_next() {
        let mut inline_comment = None;
        let mut trivialities = Vec::new();

        let mut skip_next_newline = false;
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                if inline_comment.is_none()
                    && trivialities.is_empty()
                    && text.starts_with('#')
                {
                    inline_comment = Some(text);
                    skip_next_newline = true;
                } else {
                    trivialities.push(Trivia::Comment(text));
                }
            }
            crate::children::Trivia::Whitespace(text) => {
                let mut newlines = crate::utils::count_newlines(&text);

                if skip_next_newline && newlines > 0 {
                    newlines -= 1;
                    skip_next_newline = false;
                }

                if newlines > 0 {
                    trivialities.push(Trivia::Newlines)
                }
            }
        });

        let has_inline_comment = inline_comment.is_some();
        let has_comments = trivialities
            .iter()
            .any(|trivia| matches!(trivia, Trivia::Comment(_)));
        let has_trivialities = !trivialities.is_empty();

        elements.push(Child {
            element,

            inline_comment,
            has_inline_comment,

            trivialities,
            has_comments,
            has_trivialities,
        })
    }

    elements.into_iter()
}
