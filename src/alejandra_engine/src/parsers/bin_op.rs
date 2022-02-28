use std::collections::LinkedList;

#[derive(Debug)]
pub(crate) struct BinOp {
    pub left_expression:              rnix::SyntaxElement,
    pub comments_before_operator:     LinkedList<String>,
    pub has_newlines_before_operator: bool,
    pub operator:                     rnix::SyntaxElement,
    pub comments_after_operator:      LinkedList<String>,
    pub has_newlines_after_operator:  bool,
    pub right_expression:             rnix::SyntaxElement,
}

impl BinOp {
    pub(crate) fn parse(
        build_ctx: &crate::builder::BuildCtx,
        node: &rnix::SyntaxNode,
    ) -> BinOp {
        let mut children = crate::children::Children::new(build_ctx, node);

        // left_expression
        let left_expression = children.get_next().unwrap();

        // comments_before_operator
        let mut comments_before_operator = LinkedList::new();
        let mut has_newlines_before_operator = false;
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                comments_before_operator.push_back(text);
            }
            crate::children::Trivia::Whitespace(text) => {
                has_newlines_before_operator = has_newlines_before_operator
                    || crate::utils::count_newlines(&text) > 0;
            }
        });

        // operator
        let operator = children.get_next().unwrap();

        // comments_after_operator
        let mut comments_after_operator = LinkedList::new();
        let mut has_newlines_after_operator = false;
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                comments_after_operator.push_back(text);
            }
            crate::children::Trivia::Whitespace(text) => {
                has_newlines_after_operator = has_newlines_after_operator
                    || crate::utils::count_newlines(&text) > 0;
            }
        });

        // right_expression
        let right_expression = children.get_next().unwrap();

        BinOp {
            left_expression,
            comments_before_operator,
            has_newlines_before_operator,
            operator,
            comments_after_operator,
            has_newlines_after_operator,
            right_expression,
        }
    }
}
