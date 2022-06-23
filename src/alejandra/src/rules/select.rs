pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    crate::rules::bin_op::rule_with_configuration(build_ctx, node, "select")
}
