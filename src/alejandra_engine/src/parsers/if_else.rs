use std::collections::LinkedList;

#[derive(Debug, Default)]
pub(crate) struct IfElse {
    pub comments_before_if_expr:   LinkedList<String>,
    pub if_expr:                   Option<rnix::SyntaxElement>,
    pub comments_after_if_expr:    LinkedList<String>,
    pub comments_before_then_expr: LinkedList<String>,
    pub then_expr:                 Option<rnix::SyntaxElement>,
    pub comments_after_then_expr:  LinkedList<String>,
    pub comments_before_else_expr: LinkedList<String>,
    pub else_expr:                 Option<rnix::SyntaxElement>,
}

pub(crate) fn parse(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> IfElse {
    let mut if_else = IfElse::default();

    let mut children = crate::children::Children::new(build_ctx, node);

    // if
    children.get_next().unwrap();

    // /**/
    children.drain_trivia(|element| match element {
        crate::children::Trivia::Comment(text) => {
            if_else.comments_before_if_expr.push_back(text);
        }
        crate::children::Trivia::Whitespace(_) => {}
    });

    // expr
    if_else.if_expr = Some(children.get_next().unwrap());

    // /**/
    children.drain_trivia(|element| match element {
        crate::children::Trivia::Comment(text) => {
            if_else.comments_after_if_expr.push_back(text);
        }
        crate::children::Trivia::Whitespace(_) => {}
    });

    // then
    children.get_next().unwrap();

    // /**/
    children.drain_trivia(|element| match element {
        crate::children::Trivia::Comment(text) => {
            if_else.comments_before_then_expr.push_back(text);
        }
        crate::children::Trivia::Whitespace(_) => {}
    });

    // expr
    if_else.then_expr = Some(children.get_next().unwrap());

    // /**/
    children.drain_trivia(|element| match element {
        crate::children::Trivia::Comment(text) => {
            if_else.comments_after_then_expr.push_back(text);
        }
        crate::children::Trivia::Whitespace(_) => {}
    });

    // else
    children.get_next().unwrap();

    // /**/
    children.drain_trivia(|element| match element {
        crate::children::Trivia::Comment(text) => {
            if_else.comments_before_else_expr.push_back(text);
        }
        crate::children::Trivia::Whitespace(_) => {}
    });

    // expr
    if_else.else_expr = Some(children.get_next().unwrap());

    if_else
}
