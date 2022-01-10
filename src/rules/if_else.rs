pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new(build_ctx, node);

    let layout = if children.has_comments() {
        &crate::config::Layout::Tall
    } else {
        build_ctx.config.layout()
    };

    for branch in ["if", "then", "else"] {
        // if/then/else
        let child = children.get_next().unwrap();
        steps.push_back(crate::builder::Step::Format(child.element));
        match layout {
            crate::config::Layout::Tall => {
                steps.push_back(crate::builder::Step::Indent);
            }
            crate::config::Layout::Wide => {
                steps.push_back(crate::builder::Step::Whitespace);
            }
        }

        // /**/
        children.drain_comments(|text| {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::Comment(text));
        });

        // expr
        let child = children.get_next().unwrap();
        match layout {
            crate::config::Layout::Tall => {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
                steps.push_back(crate::builder::Step::FormatWider(
                    child.element,
                ));
                if branch != "else" {
                    steps.push_back(crate::builder::Step::NewLine);
                    steps.push_back(crate::builder::Step::Pad);
                }
                steps.push_back(crate::builder::Step::Dedent);
            }
            crate::config::Layout::Wide => {
                steps.push_back(crate::builder::Step::Format(child.element));
                if branch != "else" {
                    steps.push_back(crate::builder::Step::Whitespace);
                }
            }
        }

        if branch != "else" {
            // /**/
            children.drain_comments(|text| {
                steps.push_back(crate::builder::Step::Comment(text));
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            });
        }
    }

    steps
}
