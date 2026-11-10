use alejandra::config::Config;
use alejandra::format::Status;
use pretty_assertions::assert_eq;

fn error(code: &str) -> String {
    match alejandra::format::in_memory(
        "file.nix".to_owned(),
        code.to_owned(),
        Config::default(),
    )
    .0
    {
        Status::Error(message) => message,
        Status::Changed(_) => panic!("no error for {code:?}"),
    }
}

#[test]
fn errors_show_line_and_column() {
    let cases = [
        // A missing `;`: the error is on line 4
        (
            "{pkgs, lib}: {\n  mypkg = lib.test;\n  mypkg2 = lib.test2\n  \
             mistake = {};\n}\n",
            "unexpected TOKEN_ASSIGN at line 4, column 11, wanted any of \
             [TOKEN_SEMICOLON]",
        ),
        // On the first line
        ("a b = c", "unexpected token at line 1, column 5"),
        // The column counts characters, not bytes
        ("\"ñandú\" = 1", "unexpected token at line 1, column 9"),
        // A lone carriage return ends a `#` comment, so it ends a line too
        ("# a\rx = 1", "unexpected token at line 2, column 3"),
        // Windows line endings
        ("[\r\n  1\r\n  ]]\r\n", "unexpected token at line 3, column 4"),
        // A pattern bound twice
        ("a@{b}@c: a", "unexpected double bind at line 1, column 6"),
        // No range: the message does not change
        ("[", "unexpected end of file"),
    ];

    for (code, expected) in cases {
        assert_eq!(error(code), expected, "for {code:?}");
    }
}
