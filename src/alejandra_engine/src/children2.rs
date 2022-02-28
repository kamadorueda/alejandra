use std::collections::LinkedList;

pub(crate) enum Trivia {
    Comment(String),
    Newlines(usize),
}

pub(crate) struct Child {
    pub element: rnix::SyntaxElement,

    pub inline_comment:     Option<String>,
    pub has_inline_comment: bool,

    pub trivialities:     LinkedList<Trivia>,
    pub has_comments:     bool,
    pub has_trivialities: bool,
}

pub(crate) fn new(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::linked_list::IntoIter<Child> {
    let mut children = crate::children::Children::new(build_ctx, node);

    let mut elements = LinkedList::new();

    while let Some(element) = children.get_next() {
        let mut inline_comment = None;
        let mut trivialities = LinkedList::new();

        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                if trivialities.is_empty() && text.starts_with('#') {
                    inline_comment = Some(text);
                } else {
                    trivialities.push_back(Trivia::Comment(text));
                }
            }
            crate::children::Trivia::Whitespace(text) => {
                let newlines = crate::utils::count_newlines(&text);

                if newlines > 0 {
                    trivialities.push_back(Trivia::Newlines(newlines))
                }
            }
        });

        let has_inline_comment = inline_comment.is_some();
        let has_comments = trivialities
            .iter()
            .any(|trivia| matches!(trivia, Trivia::Comment(_)));
        let has_trivialities = !trivialities.is_empty();

        elements.push_back(Child {
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
