use std::collections::LinkedList;

#[derive(Debug, Default)]
pub struct Argument {
    pub comments_before: LinkedList<String>,
    pub item:            Option<rnix::SyntaxElement>,
    pub comment_after:   Option<String>,
}

#[derive(Debug, Default)]
pub struct Pattern {
    pub initial_at:                    Option<rnix::SyntaxElement>,
    pub comments_after_initial_at:     LinkedList<String>,
    pub arguments:                     LinkedList<Argument>,
    pub comments_before_curly_b_close: LinkedList<String>,
    pub comments_before_end_at:        LinkedList<String>,
    pub end_at:                        Option<rnix::SyntaxElement>,
}

pub fn parse(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Pattern {
    let mut pattern = Pattern::default();

    let mut children = crate::children::Children::new_with_configuration(
        build_ctx, node, true,
    );

    // x @
    let child = children.peek_next().unwrap();
    if let rnix::SyntaxKind::NODE_PAT_BIND = child.element.kind() {
        pattern.initial_at = Some(child.element);
        children.move_next();
    }

    // /**/
    children.drain_trivia(|element| match element {
        crate::children::Trivia::Comment(text) => {
            pattern.comments_after_initial_at.push_back(text);
        }
        crate::children::Trivia::Whitespace(_) => {}
    });

    // {
    children.move_next();

    // arguments
    loop {
        let mut argument = Argument::default();

        // Before an item we can have: comma, comments, whitespace
        loop {
            let child = children.peek_next().unwrap();
            // eprintln!("before item {:?}", child.element.kind());

            match child.element.kind() {
                rnix::SyntaxKind::NODE_PAT_ENTRY
                | rnix::SyntaxKind::TOKEN_CURLY_B_CLOSE
                | rnix::SyntaxKind::TOKEN_ELLIPSIS => {
                    break;
                }
                rnix::SyntaxKind::TOKEN_COMMA => {
                    children.move_next();
                }
                rnix::SyntaxKind::TOKEN_COMMENT => {
                    let content =
                        child.element.into_token().unwrap().to_string();

                    argument.comments_before.push_back(content);
                    children.move_next();
                }
                rnix::SyntaxKind::TOKEN_WHITESPACE => {
                    children.move_next();
                }
                _ => {}
            }
        }

        // item
        let child = children.peek_next().unwrap();
        // eprintln!("item {:?}", child.element.kind());
        match child.element.kind() {
            rnix::SyntaxKind::TOKEN_CURLY_B_CLOSE => {
                pattern.comments_before_curly_b_close =
                    argument.comments_before;
                break;
            }
            rnix::SyntaxKind::TOKEN_ELLIPSIS
            | rnix::SyntaxKind::NODE_PAT_ENTRY => {
                argument.item = Some(child.element);
                children.move_next();
            }
            _ => {}
        }

        // After an item we can have: comma, comments, whitespace
        loop {
            let child = children.peek_next().unwrap();
            // eprintln!("after item {:?}", child.element.kind());

            match child.element.kind() {
                rnix::SyntaxKind::NODE_PAT_ENTRY
                | rnix::SyntaxKind::TOKEN_ELLIPSIS
                | rnix::SyntaxKind::TOKEN_CURLY_B_CLOSE => {
                    break;
                }
                rnix::SyntaxKind::TOKEN_COMMA => {
                    children.move_next();
                }
                rnix::SyntaxKind::TOKEN_COMMENT => {
                    let content =
                        child.element.into_token().unwrap().to_string();

                    children.move_next();
                    argument.comment_after = Some(content);
                    break;
                }
                rnix::SyntaxKind::TOKEN_WHITESPACE => {
                    let content =
                        child.element.into_token().unwrap().to_string();

                    children.move_next();
                    if crate::utils::count_newlines(&content) > 0 {
                        break;
                    }
                }
                _ => {}
            }
        }

        pattern.arguments.push_back(argument);
    }

    // }
    children.move_next();

    // /**/
    children.drain_trivia(|element| match element {
        crate::children::Trivia::Comment(text) => {
            pattern.comments_before_end_at.push_back(text);
        }
        crate::children::Trivia::Whitespace(_) => {}
    });

    // @ x
    if let Some(child) = children.peek_next() {
        if let rnix::SyntaxKind::NODE_PAT_BIND = child.element.kind() {
            pattern.end_at = Some(child.element);
        }
    }

    pattern
}
