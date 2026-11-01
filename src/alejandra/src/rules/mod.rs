pub(crate) mod apply;
pub(crate) mod attr_set;
pub(crate) mod bin_op;
pub(crate) mod dynamic;
pub(crate) mod if_else;
pub(crate) mod inherit;
pub(crate) mod key_value;
pub(crate) mod lambda;
pub(crate) mod let_in;
pub(crate) mod list;
pub(crate) mod paren;
pub(crate) mod pat_bind;
pub(crate) mod pat_entry;
pub(crate) mod pattern;
pub(crate) mod root;
pub(crate) mod scoped;
pub(crate) mod string;

pub(crate) fn default(
    _: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    node.children_with_tokens().map(crate::builder::Step::Format).collect()
}

/// Like `default`, but removes the spaces around dots: `a . b` -> `a.b`.
/// A newline next to a dot is kept, the user decides there.
/// Nodes with comments are left untouched.
pub(crate) fn dotted(
    _: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    let elements: Vec<rnix::SyntaxElement> =
        node.children_with_tokens().collect();

    let has_comments = elements
        .iter()
        .any(|element| element.kind() == rnix::SyntaxKind::TOKEN_COMMENT);

    let is_dot = |element: Option<&rnix::SyntaxElement>| {
        element.is_some_and(|element| {
            element.kind() == rnix::SyntaxKind::TOKEN_DOT
        })
    };

    elements
        .iter()
        .enumerate()
        .filter(|(index, element)| {
            has_comments
                || element.kind() != rnix::SyntaxKind::TOKEN_WHITESPACE
                || element.to_string().contains('\n')
                || !(is_dot(elements.get(index.wrapping_sub(1)))
                    || is_dot(elements.get(index + 1)))
        })
        .map(|(_, element)| {
            if element.kind() == rnix::SyntaxKind::TOKEN_WHITESPACE {
                // No trailing spaces before a kept newline
                let text = element.to_string();
                if let Some(newline) = text.find('\n') {
                    let blanks = text[..newline]
                        .trim_start_matches([' ', '\t'])
                        .len();
                    return crate::builder::Step::Token(
                        rnix::SyntaxKind::TOKEN_WHITESPACE,
                        text[newline - blanks..].to_string(),
                    );
                }
            }
            crate::builder::Step::Format(element.clone())
        })
        .collect()
}
