pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children2::new(build_ctx, node);

    let opener = children.next().unwrap();
    let expression = children.next().unwrap();
    let closer = children.next().unwrap();

    let vertical = opener.has_inline_comment
        || opener.has_trivialities
        || expression.has_inline_comment
        || expression.has_trivialities
        || closer.has_inline_comment
        || closer.has_trivialities;

    // opener
    steps.push_back(crate::builder::Step::Format(opener.element));
    if vertical {
        steps.push_back(crate::builder::Step::Indent);
    }

    if let Some(text) = opener.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else if vertical {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    for trivia in opener.trivialities {
        match trivia {
            crate::children2::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::Comment(text));
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
            crate::children2::Trivia::Newlines(_) => {}
        }
    }

    // expression
    if vertical {
        steps.push_back(crate::builder::Step::FormatWider(expression.element));
    } else {
        steps.push_back(crate::builder::Step::Format(expression.element));
    }

    if let Some(text) = expression.inline_comment {
        steps.push_back(crate::builder::Step::Whitespace);
        steps.push_back(crate::builder::Step::Comment(text));
    }

    for trivia in expression.trivialities {
        match trivia {
            crate::children2::Trivia::Comment(text) => {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::Comment(text));
            }
            crate::children2::Trivia::Newlines(_) => {}
        }
    }

    // closer
    if vertical {
        steps.push_back(crate::builder::Step::Dedent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }
    steps.push_back(crate::builder::Step::Format(closer.element));

    steps
}
