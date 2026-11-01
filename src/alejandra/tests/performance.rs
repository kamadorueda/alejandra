use std::time::Duration;
use std::time::Instant;

use alejandra::config::Config;

/// Generous bound: formatting these inputs takes milliseconds when it scales
const LIMIT: Duration = Duration::from_secs(2);

const DEPTH: usize = 30;

/// Depth for the cases that are exponential today:
/// at `DEPTH` they would take half an hour to fail
const SLOW_DEPTH: usize = 22;

fn assert_fast(name: &str, before: String) {
    let start = Instant::now();
    let (status, _) = alejandra::format::in_memory(
        name.to_owned(),
        before,
        Config::default(),
    );
    let elapsed = start.elapsed();

    assert!(
        matches!(status, alejandra::format::Status::Changed(_)),
        "`{name}` did not format"
    );
    assert!(
        elapsed < LIMIT,
        "`{name}` took {elapsed:?}, the limit is {LIMIT:?}"
    );
}

#[test]
fn nested_lists() {
    assert_fast(
        "nested_lists",
        format!("{}x{}", "[ ".repeat(DEPTH), " ]".repeat(DEPTH)),
    );
}

// Same cause as #252, but with binary operations instead of function calls
#[test]
#[ignore = "#252"]
fn nested_parens() {
    assert_fast(
        "nested_parens",
        format!("{}x{}", "(1 + ".repeat(SLOW_DEPTH), ")".repeat(SLOW_DEPTH)),
    );
}

#[test]
fn nested_let_in() {
    assert_fast(
        "nested_let_in",
        format!("{}x{}", "let a = ".repeat(DEPTH), "; in a".repeat(DEPTH)),
    );
}

#[test]
fn chained_let_in() {
    let mut before = String::new();
    for i in 0..DEPTH {
        before.push_str(&format!("let a{i} = {i}; in\n"));
    }
    before.push('x');

    assert_fast("chained_let_in", before);
}

#[test]
fn nested_attr_sets() {
    assert_fast(
        "nested_attr_sets",
        format!("{}1{}", "{ a = ".repeat(DEPTH), "; }".repeat(DEPTH)),
    );
}

// https://github.com/kamadorueda/alejandra/issues/252
#[test]
#[ignore = "#252"]
fn nested_apply() {
    assert_fast(
        "nested_apply",
        format!("{}x{}", "f (".repeat(SLOW_DEPTH), ")".repeat(SLOW_DEPTH)),
    );
}

#[test]
fn nested_lists_of_attr_sets() {
    assert_fast(
        "nested_lists_of_attr_sets",
        format!("{}x{}", "[ { a = ".repeat(DEPTH), "; } ]".repeat(DEPTH)),
    );
}

#[test]
fn nested_let_in_attr_sets() {
    assert_fast(
        "nested_let_in_attr_sets",
        format!(
            "{}1{}",
            "let a = { b = ".repeat(DEPTH),
            "; }; in a".repeat(DEPTH)
        ),
    );
}

#[test]
fn nested_lambdas() {
    assert_fast(
        "nested_lambdas",
        format!("{}x", "a: { b = ".repeat(DEPTH) + &"1; }".repeat(DEPTH)),
    );
}
