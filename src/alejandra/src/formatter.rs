use std::rc::Rc;

use nixel::cst::CST;
use nixel::deps::santiago::lexer::Lexeme;

use crate::formatting_rules;

#[derive(Clone)]
pub(crate) struct Formatter {
    buffer: String,
    current_indentation: usize,
    option_force_wide: bool,
    pub(crate) option_vertical: bool,
}

impl Formatter {
    pub(crate) fn new(
        option_force_wide: bool,
        option_vertical: bool,
    ) -> Formatter {
        Formatter {
            buffer: String::new(),
            current_indentation: 0,
            option_force_wide,
            option_vertical,
        }
    }

    pub(crate) fn dedent(&mut self) {
        self.current_indentation -= 1;
    }

    pub(crate) fn indent(&mut self) {
        self.current_indentation += 1;
    }

    pub(crate) fn finish(self) -> String {
        self.buffer
    }

    pub(crate) fn add_comment(&mut self, comment: Rc<Lexeme>) {
        let text = &comment.raw;

        if text.starts_with('#') {
            self.buffer.push_str(text.trim_end());
        } else {
            let column = comment.position.column - 1;

            let mut lines: Vec<String> = text[2..text.len() - 2]
                .lines()
                .map(|line| line.to_string())
                .collect();

            // If all lines are whitespace just return a compact comment
            if lines.iter().all(|line| line.trim().is_empty()) {
                self.buffer.push_str("/**/");
                return;
            }

            // Make sure it starts with empty line
            if lines.len() == 1 {
                lines.insert(0, "".to_string());
                lines[1] = format!("{0:<1$}{2}", "", column + 2, lines[1]);
            } else if lines[0].trim().is_empty() {
                lines[0] = "".to_string();
            } else {
                lines.insert(0, format!("{0:<1$}", "", column + 1));
                lines[1] = format!("{0:<1$}{2}", "", column + 2, lines[1]);
            }

            // Make sure it ends with empty line
            let len = lines.len();
            if len == 2 {
                lines.push(" ".repeat(column + 1));
            } else if lines[len - 1].trim().is_empty() {
                lines[len - 1] = " ".repeat(column + 1)
            } else {
                lines.push(" ".repeat(column + 1));
            }

            // Compute the distance to the first character from the left
            let mut indentation: usize = usize::MAX;
            for (index, line) in lines.iter().enumerate() {
                if index != 0 && index + 1 != lines.len() {
                    let line = line.trim_end();

                    if !line.is_empty() {
                        indentation = usize::min(
                            indentation,
                            line.len() - line.trim_start().len(),
                        );
                    }
                }
            }
            if indentation == usize::MAX {
                indentation = column;
            };

            // Re-align everything with respect to the vertical
            lines = lines
                .iter()
                .enumerate()
                .map(|(index, line)| {
                    if index == 0 || index + 1 == lines.len() {
                        line.to_string()
                    } else if column >= indentation {
                        format!(
                            "{0:<1$}{2}",
                            "",
                            column - indentation + 1,
                            line,
                        )
                    } else if line.len() >= indentation - column {
                        line[indentation - column - 1..line.len()].to_string()
                    } else {
                        line.to_string()
                    }
                })
                .collect();

            // Dedent everything as much as possible so that upstream components
            // can indent as they see convenient
            lines = lines
                .iter()
                .enumerate()
                .map(|(index, line)| {
                    if index == 0 {
                        line.to_string()
                    } else if line.len() > column {
                        line[column + 1..line.len()].to_string()
                    } else {
                        line.to_string()
                    }
                })
                .collect();

            // Print it
            self.buffer.push_str("/*");

            for (index, line) in
                lines.iter().map(|line| line.trim_end()).enumerate()
            {
                if index == 0 {
                    self.buffer.push_str(line);
                } else if !line.is_empty() {
                    self.buffer.push(' ');
                    for _ in 0..self.current_indentation {
                        self.buffer.push_str("  ");
                    }
                    self.buffer.push_str(line);
                }

                if index + 1 != lines.len() {
                    self.buffer.push('\n')
                }
            }

            self.add_padding();
            self.buffer.push_str(" */");
        }
    }

    pub(crate) fn add_lexeme(&mut self, lexeme: Rc<Lexeme>) -> Result<(), ()> {
        self.buffer.push_str(&lexeme.raw);
        Ok(())
    }

    pub(crate) fn add_newline(&mut self) -> Result<(), ()> {
        if self.option_force_wide {
            Err(())
        } else {
            self.buffer.push('\n');

            Ok(())
        }
    }

    pub(crate) fn add_padding(&mut self) {
        if self.current_indentation > 0 {
            self.buffer.reserve(2 * self.current_indentation);
            for _ in 0..self.current_indentation {
                self.buffer.push_str("  ");
            }
        }
    }

    pub(crate) fn add_ws(&mut self) {
        self.buffer.push(' ');
    }

    pub(crate) fn format(&mut self, cst: CST) -> Result<(), ()> {
        match cst {
            CST::Assert {
                assert,
                assert_trivia,
                expression,
                expression_trivia,
                semicolon,
                semicolon_trivia,
                target,
            } => {
                formatting_rules::assert::rule(
                    self,
                    assert,
                    assert_trivia,
                    expression,
                    expression_trivia,
                    semicolon,
                    semicolon_trivia,
                    target,
                )
            },
            CST::AttributePath { parts } => {
                formatting_rules::attribute_path::rule(self, parts)
            },
            CST::BinaryOperation {
                left_operand,
                left_operand_trivia,
                operator,
                operator_trivia,
                right_operand,
            } => {
                formatting_rules::binary_operation::rule(
                    self,
                    left_operand,
                    left_operand_trivia,
                    operator,
                    operator_trivia,
                    right_operand,
                )
            },
            CST::Binding {
                from,
                from_trivia,
                equal,
                equal_trivia,
                to,
                to_trivia,
                semicolon,
            } => {
                formatting_rules::binding::rule(
                    self,
                    from,
                    from_trivia,
                    equal,
                    equal_trivia,
                    to,
                    to_trivia,
                    semicolon,
                )
            },
            CST::Float { lexeme } => self.add_lexeme(lexeme),
            CST::FunctionApplication {
                function,
                function_trivia,
                arguments,
            } => {
                formatting_rules::function_application::rule(
                    self,
                    function,
                    function_trivia,
                    arguments,
                )
            },
            CST::FunctionDestructured {
                identifier: _,
                open: _,
                open_trivia: _,
                arguments: _,
                ellipsis: _,
                close: _,
                close_trivia: _,
                vertical_two_dots: _,
                vertical_two_dots_trivia: _,
                definition: _,
            } => {
                self.buffer.push_str("CST::FunctionDestructured");
                Ok(())
            },
            CST::FunctionSimple {
                argument: _,
                argument_trivia: _,
                vertical_two_dots: _,
                vertical_two_dots_trivia: _,
                definition: _,
            } => {
                self.buffer.push_str("CST::FunctionSimple");
                Ok(())
            },
            CST::HasProperty {
                expression: _,
                expression_trivia: _,
                question: _,
                question_trivia: _,
                attribute_path: _,
            } => {
                self.buffer.push_str("CST::HasProperty");
                Ok(())
            },
            CST::IfThenElse {
                if_: _,
                if_trivia: _,
                predicate: _,
                predicate_trivia: _,
                then: _,
                then_trivia: _,
                then_expression: _,
                then_expression_trivia: _,
                else_: _,
                else_trivia: _,
                else_expression: _,
            } => {
                self.buffer.push_str("CST::IfThenElse");
                Ok(())
            },
            CST::Inherit {
                inherit: _,
                inherit_trivia: _,
                attributes: _,
                semicolon: _,
            } => {
                self.buffer.push_str("CST::Inherit");
                Ok(())
            },
            CST::InheritFrom {
                inherit: _,
                inherit_trivia: _,
                from: _,
                from_trivia: _,
                attributes: _,
                semicolon: _,
            } => {
                self.buffer.push_str("CST::InheritFrom");
                Ok(())
            },
            CST::Int { lexeme } => self.add_lexeme(lexeme),
            CST::LetIn {
                let_: _,
                let_trivia: _,
                bindings: _,
                in_: _,
                in_trivia: _,
                target: _,
            } => {
                self.buffer.push_str("CST::LetIn");
                Ok(())
            },
            CST::List { open, open_trivia, elements, close } => {
                formatting_rules::list::rule(
                    self,
                    open,
                    open_trivia,
                    elements,
                    close,
                )
            },
            CST::Map { recursive, open, open_trivia, bindings, close } => {
                formatting_rules::map::rule(
                    self,
                    recursive,
                    open,
                    open_trivia,
                    bindings,
                    close,
                )
            },
            CST::Parentheses {
                open,
                open_trivia,
                expression,
                expression_trivia,
                close,
            } => {
                formatting_rules::parentheses::rule(
                    self,
                    open,
                    open_trivia,
                    expression,
                    expression_trivia,
                    close,
                )
            },
            CST::PartInterpolation {
                open,
                open_trivia,
                expression,
                expression_trivia,
                close,
            } => {
                self.indent();
                formatting_rules::parentheses::rule(
                    self,
                    open,
                    open_trivia,
                    expression,
                    expression_trivia,
                    close,
                )?;
                self.dedent();
                Ok(())
            },
            CST::PartRaw { lexeme } => self.add_lexeme(lexeme),
            CST::Path { parts: _ } => {
                self.buffer.push_str("CST::Path");
                Ok(())
            },
            CST::PropertyAccess {
                expression,
                expression_trivia,
                dot,
                dot_trivia,
                attribute_path,
            } => {
                formatting_rules::property_access::rule(
                    self,
                    expression,
                    expression_trivia,
                    dot,
                    dot_trivia,
                    attribute_path,
                )
            },
            CST::PropertyAccessWithDefault {
                expression: _,
                expression_trivia: _,
                dot: _,
                dot_trivia: _,
                attribute_path: _,
                attribute_path_trivia: _,
                or: _,
                or_trivia: _,
                default: _,
            } => {
                self.buffer.push_str("CST::PropertyAccessWithDefault");
                Ok(())
            },
            CST::Root { trivia_before, expression, trivia_after } => {
                formatting_rules::root::rule(
                    self,
                    trivia_before,
                    expression,
                    trivia_after,
                )
            },
            CST::SearchNixPath { lexeme } => self.add_lexeme(lexeme),
            CST::String { open, parts, close } => {
                formatting_rules::string::rule(self, open, parts, close)
            },
            CST::UnaryOperation {
                operator: _,
                operator_trivia: _,
                operand: _,
            } => {
                self.buffer.push_str("CST::UnaryOperation");
                Ok(())
            },
            CST::Variable { lexeme } => self.add_lexeme(lexeme),
            CST::With {
                with: _,
                with_trivia: _,
                expression: _,
                expression_trivia: _,
                semicolon: _,
                semicolon_trivia: _,
                target: _,
            } => {
                self.buffer.push_str("CST::With");
                Ok(())
            },
        }
    }

    pub(crate) fn format_wider(&mut self, cst: CST) -> Result<(), ()> {
        self.option_vertical = !self.fits_in_single_line(cst.clone());
        self.format(cst)
    }

    pub(crate) fn fits_in_single_line(&mut self, cst: CST) -> bool {
        let mut formatter = Formatter::new(true, self.option_vertical);

        formatter.format(cst).is_ok()
    }
}
