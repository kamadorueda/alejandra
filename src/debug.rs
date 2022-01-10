pub fn display(
    element: &rowan::NodeOrToken<&rowan::GreenNodeData, &rowan::GreenTokenData>,
) {
    eprintln!("AST:");
    display_recursive(element, 2);
}

fn display_recursive(
    element: &rowan::NodeOrToken<&rowan::GreenNodeData, &rowan::GreenTokenData>,
    depth: usize,
) {
    let kind = unsafe {
        std::mem::transmute::<u16, rnix::SyntaxKind>(element.kind().0)
    };

    match element {
        rowan::NodeOrToken::Node(node) => {
            eprintln!("{0:<1$}{2:?}", "", 2 * depth, kind);
            for child in node.children() {
                display_recursive(&child, depth + 1);
            }
        }
        rowan::NodeOrToken::Token(token) => {
            eprintln!("{0:<1$}{2:?} {3:?}", "", 2 * depth, kind, token.text());
        }
    }
}
