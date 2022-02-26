pub(crate) fn has_newlines(string: &str) -> bool {
    string.chars().any(|c| c == '\n')
}

pub(crate) fn count_newlines(string: &str) -> usize {
    string.chars().filter(|c| *c == '\n').count()
}
