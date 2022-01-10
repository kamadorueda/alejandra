#[derive(Clone, Debug)]
pub struct Position {
    pub column: usize,
    pub line:   usize,
}

impl Position {
    pub fn new() -> Position {
        Position { column: 0, line: 1 }
    }

    pub fn update(&mut self, text: &str) {
        let chars: Vec<char> = text.chars().collect();
        let newlines = chars.iter().filter(|&c| *c == '\n').count();
        self.line += newlines;
        if newlines > 0 {
            self.column = 0
        }
        self.column += match chars.iter().rposition(|c| *c == '\n') {
            Some(pos) => chars.len() - pos,
            None => chars.len(),
        };
    }
}
