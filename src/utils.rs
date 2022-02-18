pub fn has_newlines(string: &str) -> bool {
    string.chars().any(|c| c == '\n')
}

pub fn count_newlines(string: &str) -> usize {
    string.chars().filter(|c| *c == '\n').count()
}
