use std::collections::LinkedList;

#[derive(Debug)]
struct Argument {
    comments_before: LinkedList<String>,
    item:            Option<rnix::SyntaxElement>,
    comment_after:   Option<String>,
}

impl Default for Argument {
    fn default() -> Argument {
        Argument {
            comments_before: LinkedList::new(),
            item:            None,
            comment_after:   None,
        }
    }
}

#[derive(Debug)]
struct Pattern {
    initial_at:                    Option<rnix::SyntaxElement>,
    comments_after_initial_at:     LinkedList<String>,
    arguments:                     LinkedList<Argument>,
    comments_before_curly_b_close: LinkedList<String>,
    comments_before_end_at:        LinkedList<String>,
    end_at:                        Option<rnix::SyntaxElement>,
}

impl Default for Pattern {
    fn default() -> Pattern {
        Pattern {
            initial_at:                    None,
            comments_after_initial_at:     LinkedList::new(),
            arguments:                     LinkedList::new(),
            comments_before_curly_b_close: LinkedList::new(),
            comments_before_end_at:        LinkedList::new(),
            end_at:                        None,
        }
    }
}

fn parse(
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
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            pattern.comments_after_initial_at.push_back(text);
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
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

    // /**/
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            pattern.comments_before_curly_b_close.push_back(text);
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
    });

    // }
    children.move_next();

    // /**/
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            pattern.comments_before_end_at.push_back(text);
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
    });

    // @ x
    if let Some(child) = children.peek_next() {
        if let rnix::SyntaxKind::NODE_PAT_BIND = child.element.kind() {
            pattern.end_at = Some(child.element);
        }
    }

    pattern
}

pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let pattern = parse(build_ctx, node);

    let has_comments_between_curly_b =
        pattern.arguments.iter().any(|argument| {
            argument.comment_after.is_some()
                || !argument.comments_before.is_empty()
        });

    let has_comments = has_comments_between_curly_b
        || !pattern.comments_after_initial_at.is_empty()
        || !pattern.comments_before_end_at.is_empty();

    let has_ellipsis = pattern.arguments.iter().any(|argument| {
        if argument.item.is_some() {
            argument.item.as_ref().unwrap().kind()
                == rnix::SyntaxKind::TOKEN_ELLIPSIS
        } else {
            false
        }
    });

    let arguments_count = pattern.arguments.len();

    let arguments_count_for_tall = if has_ellipsis { 2 } else { 1 };

    let layout = if has_comments
        || arguments_count > arguments_count_for_tall
        || (arguments_count > 0 && has_comments_between_curly_b)
    {
        &crate::config::Layout::Tall
    } else {
        build_ctx.config.layout()
    };

    // x @
    if let Some(element) = &pattern.initial_at {
        let element = element.clone();
        match layout {
            crate::config::Layout::Tall => {
                steps.push_back(crate::builder::Step::FormatWider(element));
            }
            crate::config::Layout::Wide => {
                steps.push_back(crate::builder::Step::Format(element));
            }
        }
    }

    // /**/
    if !pattern.comments_after_initial_at.is_empty() {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        for text in pattern.comments_after_initial_at {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
    } else if pattern.initial_at.is_some() {
        steps.push_back(crate::builder::Step::Whitespace);
    }

    // {
    steps.push_back(crate::builder::Step::Token(
        rnix::SyntaxKind::TOKEN_CURLY_B_OPEN,
        "{".to_string(),
    ));
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::Indent);
        }
        crate::config::Layout::Wide => {}
    };

    // arguments
    let mut index = 0;
    for argument in pattern.arguments {
        match layout {
            crate::config::Layout::Tall => {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
            crate::config::Layout::Wide => {
                if index > 0 {
                    steps.push_back(crate::builder::Step::Whitespace);
                }
            }
        }

        // /**/
        if !argument.comments_before.is_empty() {
            for text in argument.comments_before {
                steps.push_back(crate::builder::Step::Comment(text));
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
        }

        // argument
        let element = argument.item.unwrap();
        let element_kind = element.kind();
        match layout {
            crate::config::Layout::Tall => {
                steps.push_back(crate::builder::Step::FormatWider(element));
            }
            crate::config::Layout::Wide => {
                steps.push_back(crate::builder::Step::Format(element));
            }
        };

        // ,
        match layout {
            crate::config::Layout::Tall => {
                if !matches!(element_kind, rnix::SyntaxKind::TOKEN_ELLIPSIS) {
                    steps.push_back(crate::builder::Step::Token(
                        rnix::SyntaxKind::TOKEN_COMMA,
                        ",".to_string(),
                    ));
                }
            }
            crate::config::Layout::Wide => {
                if index + 1 < arguments_count {
                    steps.push_back(crate::builder::Step::Token(
                        rnix::SyntaxKind::TOKEN_COMMA,
                        ",".to_string(),
                    ));
                }
            }
        };

        // possible inline comment
        if let Some(text) = argument.comment_after {
            if text.starts_with('#') {
                steps.push_back(crate::builder::Step::Whitespace);
            } else {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
            steps.push_back(crate::builder::Step::Comment(text));
        }

        index += 1;
    }

    // /**/
    let has_comments_before_curly_b_close =
        !pattern.comments_before_curly_b_close.is_empty();
    for text in pattern.comments_before_curly_b_close {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        steps.push_back(crate::builder::Step::Comment(text));
    }

    // }
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::Dedent);
            if arguments_count > 0 || has_comments_before_curly_b_close {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
        }
        crate::config::Layout::Wide => {}
    };
    steps.push_back(crate::builder::Step::Token(
        rnix::SyntaxKind::TOKEN_CURLY_B_OPEN,
        "}".to_string(),
    ));

    // /**/
    if pattern.comments_before_end_at.is_empty() {
        if pattern.end_at.is_some() {
            steps.push_back(crate::builder::Step::Whitespace);
        }
    } else {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        for text in pattern.comments_before_end_at {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
    }

    // @ x
    if let Some(element) = pattern.end_at {
        match layout {
            crate::config::Layout::Tall => {
                steps.push_back(crate::builder::Step::FormatWider(element));
            }
            crate::config::Layout::Wide => {
                steps.push_back(crate::builder::Step::Format(element));
            }
        }
    }

    steps
}
