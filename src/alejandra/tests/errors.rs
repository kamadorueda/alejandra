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
        // A missing `;` at the end of line 3: the parser sees it at the `=`
        // of line 4, so the message also says where line 3 ends
        (
            "{pkgs, lib}: {\n  mypkg = lib.test;\n  mypkg2 = lib.test2\n  \
             mistake = {};\n}\n",
            "unexpected TOKEN_ASSIGN at line 4, column 11 (after line 3, \
             column 20), wanted any of [TOKEN_SEMICOLON]",
        ),
        // Comments and empty lines between do not count
        (
            "{\n  a = 1 # one\n\n  # two\n  b = 2;\n}\n",
            "unexpected TOKEN_ASSIGN at line 5, column 5 (after line 2, \
             column 7), wanted any of [TOKEN_SEMICOLON]",
        ),
        // The column of the code before counts characters too
        (
            "{\n  a = \"ñ\"\n  b = 2;\n}\n",
            "unexpected TOKEN_ASSIGN at line 3, column 5 (after line 2, \
             column 9), wanted any of [TOKEN_SEMICOLON]",
        ),
        // The code before is on the same line: nothing more to say
        (
            "{ a = 1 b = 2; }",
            "unexpected TOKEN_ASSIGN at line 1, column 11, wanted any of \
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
