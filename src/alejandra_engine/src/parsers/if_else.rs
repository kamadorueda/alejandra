use std::collections::LinkedList;

#[derive(Debug)]
pub(crate) struct IfElse {
    pub if_:                       rnix::SyntaxElement,
    pub comments_before_if_expr:   LinkedList<String>,
    pub if_expr:                   rnix::SyntaxElement,
    pub comments_after_if_expr:    LinkedList<String>,
    pub then_:                     rnix::SyntaxElement,
    pub comments_before_then_expr: LinkedList<String>,
    pub then_expr:                 rnix::SyntaxElement,
    pub comments_after_then_expr:  LinkedList<String>,
    pub else_:                     rnix::SyntaxElement,
    pub comments_before_else_expr: LinkedList<String>,
    pub else_expr:                 rnix::SyntaxElement,
}

impl IfElse {
    pub(crate) fn parse(
        build_ctx: &crate::builder::BuildCtx,
        node: &rnix::SyntaxNode,
    ) -> IfElse {
        let mut children = crate::children::Children::new(build_ctx, node);

        // if_
        let if_ = children.get_next().unwrap();

        // comments_before_if_expr
        let mut comments_before_if_expr = LinkedList::new();
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                comments_before_if_expr.push_back(text);
            }
            crate::children::Trivia::Whitespace(_) => {}
        });

        // if_expr
        let if_expr = children.get_next().unwrap();

        // comments_after_if_expr
        let mut comments_after_if_expr = LinkedList::new();
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                comments_after_if_expr.push_back(text);
            }
            crate::children::Trivia::Whitespace(_) => {}
        });

        // then_
        let then_ = children.get_next().unwrap();

        // comments_before_then_expr
        let mut comments_before_then_expr = LinkedList::new();
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                comments_before_then_expr.push_back(text);
            }
            crate::children::Trivia::Whitespace(_) => {}
        });

        // then_expr
        let then_expr = children.get_next().unwrap();

        // comments_after_then_expr
        let mut comments_after_then_expr = LinkedList::new();
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                comments_after_then_expr.push_back(text);
            }
            crate::children::Trivia::Whitespace(_) => {}
        });

        // else_
        let else_ = children.get_next().unwrap();

        // comments_before_else_expr
        let mut comments_before_else_expr = LinkedList::new();
        children.drain_trivia(|element| match element {
            crate::children::Trivia::Comment(text) => {
                comments_before_else_expr.push_back(text);
            }
            crate::children::Trivia::Whitespace(_) => {}
        });

        // else_expr
        let else_expr = children.get_next().unwrap();

        IfElse {
            if_,
            comments_before_if_expr,
            if_expr,
            comments_after_if_expr,
            then_,
            comments_before_then_expr,
            then_expr,
            comments_after_then_expr,
            else_,
            comments_before_else_expr,
            else_expr,
        }
    }
}
