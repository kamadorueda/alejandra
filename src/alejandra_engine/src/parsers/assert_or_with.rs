use std::collections::LinkedList;

#[derive(Debug)]
pub(crate) struct AssertOrWith {
    pub assert_or_with:                    rnix::SyntaxElement,
    pub comments_after_assert_or_with:     LinkedList<String>,
    pub has_newlines_after_assert_or_with: bool,
    pub first_expression:                  rnix::SyntaxElement,
    pub semicolon:                         rnix::SyntaxElement,
    pub comments_after_semicolon:          LinkedList<String>,
    pub has_newlines_after_semicolon:      bool,
    pub second_expression:                 rnix::SyntaxElement,
}

impl AssertOrWith {
    pub(crate) fn new(
        build_ctx: &crate::builder::BuildCtx,
        node: &rnix::SyntaxNode,
    ) -> AssertOrWith {
        let mut children = crate::children::Children::new(build_ctx, node);

        // assert_or_with
        let assert_or_with = children.get_next().unwrap();

        // comments_after_assert_or_with
        // has_newlines_after_assert_or_with
        let mut comments_after_assert_or_with = LinkedList::new();
        let mut has_newlines_after_assert_or_with = false;
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                comments_after_assert_or_with.push_back(text);
            }
            crate::children::Trivia::Whitespace(text) => {
                has_newlines_after_assert_or_with =
                    has_newlines_after_assert_or_with
                        || crate::utils::count_newlines(&text) > 0;
            }
        });

        // first_expression
        let first_expression = children.get_next().unwrap();

        // semicolon
        let semicolon = children.get_next().unwrap();

        // comments_after_semicolon
        // has_newlines_after_semicolon
        let mut comments_after_semicolon = LinkedList::new();
        let mut has_newlines_after_semicolon = false;
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                comments_after_semicolon.push_back(text);
            }
            crate::children::Trivia::Whitespace(text) => {
                has_newlines_after_semicolon = has_newlines_after_semicolon
                    || crate::utils::count_newlines(&text) > 0;
            }
        });

        // second_expression
        let second_expression = children.get_next().unwrap();

        AssertOrWith {
            assert_or_with,
            comments_after_assert_or_with,
            has_newlines_after_assert_or_with,
            first_expression,
            semicolon,
            comments_after_semicolon,
            has_newlines_after_semicolon,
            second_expression,
        }
    }
}
