#[derive(Clone)]
pub struct Child {
    pub element: rnix::SyntaxElement,
    pub pos:     crate::position::Position,
}

pub struct Children {
    children:      Vec<Child>,
    current_index: usize,
}

impl Children {
    pub fn new(
        build_ctx: &crate::builder::BuildCtx,
        node: &rnix::SyntaxNode,
    ) -> Children {
        let mut children = Vec::new();

        let mut pos = build_ctx.pos_old.clone();

        for child in node.children_with_tokens() {
            match child {
                rnix::SyntaxElement::Node(_) => {
                    children
                        .push(Child { element: child, pos: pos.clone() });
                }
                rnix::SyntaxElement::Token(token) => {
                    match token.kind() {
                        rnix::SyntaxKind::TOKEN_WHITESPACE => {}
                        _ => {
                            children.push(Child {
                                element: token.clone().into(),
                                pos:     pos.clone(),
                            });
                        }
                    }

                    pos.update(token.text());
                }
            }
        }

        Children { children, current_index: 0 }
    }

    pub fn count(&self) -> usize {
        self.children.len()
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn get(&mut self, index: usize) -> Option<Child> {
        if index + 1 > self.children.len() {
            None
        } else {
            Some(self.children[index].clone())
        }
    }

    pub fn get_next(&mut self) -> Option<Child> {
        let child = self.get(self.current_index);
        self.move_next();
        child
    }

    pub fn has_next(&self) -> bool {
        self.current_index < self.children.len()
    }

    pub fn peek_next(&mut self) -> Option<Child> {
        self.get(self.current_index)
    }

    pub fn peek_prev(&mut self) -> Option<Child> {
        if self.current_index >= 1 {
            self.get(self.current_index - 1)
        } else {
            None
        }
    }

    pub fn move_next(&mut self) {
        self.current_index += 1
    }

    pub fn move_prev(&mut self) {
        self.current_index -= 1
    }

    pub fn has_comments(&self) -> bool {
        self.children.iter().any(|child| {
            child.element.kind() == rnix::SyntaxKind::TOKEN_COMMENT
        })
    }

    pub fn drain_comment<F: FnMut(String)>(&mut self, mut callback: F) {
        if let Some(child) = self.peek_next() {
            match child.element.kind() {
                rnix::SyntaxKind::TOKEN_COMMENT => {
                    callback(dedent_comment(
                        &child.pos,
                        child.element.into_token().unwrap().text(),
                    ));
                    self.move_next();
                }
                _ => {}
            }
        }
    }

    pub fn drain_comments<F: FnMut(String)>(&mut self, mut callback: F) {
        while let Some(child) = self.peek_next() {
            match child.element.kind() {
                rnix::SyntaxKind::TOKEN_COMMENT => {
                    callback(dedent_comment(
                        &child.pos,
                        child.element.into_token().unwrap().text(),
                    ));
                    self.move_next();
                }
                _ => {
                    break;
                }
            }
        }
    }
}

fn dedent_comment(pos: &crate::position::Position, text: &str) -> String {
    if text.starts_with("#") {
        text.to_string()
    } else {
        let text = text[2..text.len() - 2]
            .lines()
            .enumerate()
            .map(|(index, line)| {
                if index > 0 {
                    line.chars()
                        .skip(if pos.column >= 1 { pos.column - 1 } else { 0 })
                        .collect::<String>()
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!("/*{}*/", text)
    }
}

// fn dedent_string(
//     pos: &crate::position::Position,
//     node: &rnix::SyntaxNode,
// ) -> String {
//     eprintln!("{}", text);
//     if text.starts_with("\"") {
//         text.to_string()
//     } else {
//         node.children_with_tokens().filter(|child| {
//             child.kind() == rnix::SyntaxKind::TOKEN_STRING_CONTENT
//         }).map(|child| {
//             let lines = child.into_token().unwrap().lines();
//             ""
//             "                    rustup toolchain install nightly"
//             "                    "
//             ""
//             "                  "
//         });

//         // let padding_to_first_char = lines
//         // TOKEN_STRING_CONTENT

//         let text = text[2..text.len() - 2]
//             .lines()
//             .enumerate()
//             .map(|(index, line)| {
//                 if index > 0 {
//                     line.chars()
//                         .skip(if pos.column >= 1 { pos.column - 1 } else { 0 })
//                         .collect::<String>()
//                 } else {
//                     line.to_string()
//                 }
//             })
//             .collect::<Vec<String>>()
//             .join("\n");

//         format!("/*{}*/", text)
//     }
// }
