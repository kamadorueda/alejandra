pub mod apply;
pub mod assert_and_with;
pub mod attr_set;
pub mod bin_op_and_or_default;
pub mod dynamic;
pub mod if_else;
pub mod inherit;
pub mod key_value;
pub mod lambda;
pub mod let_in;
pub mod list;
pub mod paren;
pub mod pat_bind;
pub mod pat_entry;
pub mod pattern;
pub mod root;
pub mod select;
pub mod string;
pub mod string_interpol;

pub fn default(
    _: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    node.children_with_tokens()
        .map(|child| crate::builder::Step::Format(child.into()))
        .collect()
}
