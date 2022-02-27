use std::collections::LinkedList;

#[derive(Debug)]
pub(crate) struct Apply {
    pub left_expression:         rnix::SyntaxElement,
    pub comments_after_left:     LinkedList<String>,
    pub has_newlines_after_left: bool,
    pub right_expression:        rnix::SyntaxElement,
}

impl Apply {
    pub(crate) fn parse(
        build_ctx: &crate::builder::BuildCtx,
        node: &rnix::SyntaxNode,
    ) -> Apply {
        let mut children = crate::children::Children::new(build_ctx, node);

        // left_expression
        let left_expression = children.get_next().unwrap();

        // comments_after_left
        // has_newlines_after_left
        let mut comments_after_left = LinkedList::new();
        let mut has_newlines_after_left = false;
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                comments_after_left.push_back(text);
            }
            crate::children::Trivia::Whitespace(text) => {
                has_newlines_after_left = has_newlines_after_left
                    || crate::utils::count_newlines(&text) > 0;
            }
        });

        // right_expression
        let right_expression = children.get_next().unwrap();

        Apply {
            left_expression,
            comments_after_left,
            has_newlines_after_left,
            right_expression,
        }
    }
}
