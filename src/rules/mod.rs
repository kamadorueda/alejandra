pub mod apply;
pub mod assert;
pub mod attr_set;
pub mod bin_op;
pub mod dynamic;
pub mod if_else;
pub mod inherit;
pub mod key_value;
pub mod lambda;
pub mod let_in;
pub mod list;
pub mod or_default;
pub mod paren;
pub mod pat_bind;
pub mod pat_entry;
pub mod pattern;
pub mod root;
pub mod select;
pub mod string;
pub mod string_interpol;
pub mod with;

pub fn default(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new(build_ctx, node);

    while let Some(child) = children.get_next() {
        let step = match build_ctx.config.layout() {
            crate::config::Layout::Tall => {
                crate::builder::Step::FormatWider(child.element)
            }
            _ => crate::builder::Step::Format(child.element),
        };
        steps.push_back(step);
    }

    steps
}
