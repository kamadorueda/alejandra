use nixel::cst::CST;

use crate::formatter::Formatter;

pub(crate) fn count_newlines(string: &str) -> usize {
    string.chars().filter(|c| *c == '\n').count()
}

pub(crate) fn second_through_penultimate_line_are_indented(
    formatter: &Formatter,
    cst: CST,
    if_leq_than_two_lines: bool,
) -> bool {
    let mut formatter = Formatter::new(false, formatter.option_vertical);

    formatter.format(cst).unwrap();

    let formatted = formatter.finish();

    let formatted_lines: Vec<&str> = formatted.split('\n').collect();

    if formatted_lines.len() <= 2 {
        return if_leq_than_two_lines;
    }

    formatted_lines.iter().skip(1).rev().skip(1).all(|line| {
        line.is_empty()
            || line.starts_with("}:")
            || line.starts_with("in")
            || line.starts_with("  ")
    })
}
