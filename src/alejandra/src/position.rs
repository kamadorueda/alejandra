#[derive(Clone)]
pub(crate) struct Position {
    pub column: usize,
    pub line: usize,
}

impl Default for Position {
    fn default() -> Position {
        Position { column: 0, line: 1 }
    }
}

impl Position {
    pub fn update(&mut self, text: &str) {
        for char in text.chars() {
            if char == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
    }
}
