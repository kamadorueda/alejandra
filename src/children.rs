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
        let mut lines: Vec<String> = text[2..text.len() - 2]
            .lines()
            .map(|line| line.to_string())
            .collect();

        // If all lines are whitespace just return a compact comment
        if lines.iter().all(|line| line.trim().len() == 0) {
            return "/**/".to_string();
        }

        // println!("{:?}", lines);
        // println!("0\n{0:<1$}/*{2}*/\n", "", pos.column, lines.join("\n"));

        if lines.len() == 1 {
            lines.insert(0, "".to_string());
            lines[1] = format!("{0:<1$}{2}", "", pos.column + 2, lines[1]);
        } else if lines[0].trim().len() == 0 {
            lines[0] = "".to_string();
        } else {
            lines.insert(0, format!("{0:<1$}", "", pos.column + 1));
            lines[1] = format!("{0:<1$}{2}", "", pos.column + 2, lines[1]);
        }

        // println!("{:?}", lines);
        // println!("1\n{0:<1$}/*{2}*/\n", "", pos.column, lines.join("\n"));

        let len = lines.len();
        if len == 2 {
            lines.push(format!("{0:<1$}", "", pos.column + 1));
        } else if lines[len - 1].trim().len() == 0 {
            lines[len - 1] = format!("{0:<1$}", "", pos.column + 1)
        } else {
            // lines[len - 1] =
            //     format!("{0:<1$}{2}", "", pos.column + 2, lines[len - 1]);
            lines.push(format!("{0:<1$}", "", pos.column + 1));
        }

        // println!("{:?}", lines);
        // println!("2\n{0:<1$}/*{2}*/\n", "", pos.column, lines.join("\n"));

        let mut indentation: usize = usize::MAX;
        for (index, line) in lines.iter().enumerate() {
            if index != 0 && index + 1 != lines.len() {
                let line = line.trim_end();

                if line.len() > 0 {
                    indentation = usize::min(
                        indentation,
                        line.len() - line.trim_start().len(),
                    );
                }
            }
        }
        if indentation == usize::MAX {
            indentation = pos.column;
        };

        lines = lines
            .iter()
            .enumerate()
            .map(|(index, line)| {
                if index == 0 || index + 1 == lines.len() {
                    line.to_string()
                } else {
                    if pos.column >= indentation {
                        format!(
                            "{0:<1$}{2}",
                            "",
                            pos.column - indentation + 1,
                            line,
                        )
                    } else if line.len() >= indentation - pos.column {
                        line[indentation - pos.column - 1..line.len()]
                            .to_string()
                    } else {
                        line.to_string()
                    }
                }
            })
            .collect();

        // println!("{:?}", lines);
        // println!("3\n{0:<1$}/*{2}*/\n", "", pos.column, lines.join("\n"));
        // println!("indentation={} pos.column{}", indentation, pos.column);

        lines = lines
            .iter()
            .enumerate()
            .map(|(index, line)| {
                if index == 0 {
                    line.to_string()
                } else if line.len() >= pos.column + 1 {
                    line[pos.column + 1..line.len()].to_string()
                } else {
                    line.to_string()
                }
            })
            .collect();

        // println!("{:?}", lines);
        // println!("4\n{0:<1$}/*{2}*/\n", "", pos.column, lines.join("\n"));

        format!("/*{}*/", lines.join("\n"))
    }
}
