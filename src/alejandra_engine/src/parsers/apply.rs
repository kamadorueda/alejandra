use std::collections::LinkedList;

#[derive(Debug, Default)]
pub(crate) struct Apply {
    pub left:     Option<rnix::SyntaxElement>,
    pub comments: LinkedList<String>,
    pub newline:  bool,
    pub right:    Option<rnix::SyntaxElement>,
}

pub(crate) fn parse(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Apply {
    let mut apply = Apply::default();

    let mut children = crate::children::Children::new(build_ctx, node);

    // left
    apply.left = Some(children.get_next().unwrap());

    // /**/
    children.drain_trivia(|element| match element {
        crate::children::Trivia::Comment(text) => {
            apply.comments.push_back(text);
        }
        crate::children::Trivia::Whitespace(text) => {
            if !apply.newline {
                apply.newline = crate::utils::count_newlines(&text) > 0;
            }
        }
    });

    // right
    apply.right = Some(children.get_next().unwrap());

    apply
}
