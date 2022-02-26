#[derive(Clone, Debug)]
pub(crate) struct Position {
    pub column: usize,
    pub line:   usize,
}

impl Default for Position {
    fn default() -> Position {
        Position { column: 0, line: 1 }
    }
}

impl Position {
    pub fn update(&mut self, text: &str) {
        let chars: Vec<char> = text.chars().collect();
        let newlines = chars.iter().filter(|&c| *c == '\n').count();
        self.line += newlines;
        if newlines > 0 {
            self.column = 0
        }
        self.column += match chars.iter().rposition(|c| *c == '\n') {
            Some(pos) => chars.len() - pos - 1,
            None => chars.len(),
        };
    }
}
