use std::fmt::Write as _;
use std::io::Write as _;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

#[derive(Debug)]
struct TestCase {
    args:  &'static [&'static str],
    stdin: Option<&'static str>,
}

const CASES: &[TestCase] = &[
    TestCase { args: &["--help"], stdin: None },
    TestCase { args: &["--version"], stdin: None },
    TestCase { args: &[], stdin: None },
    TestCase { args: &["--quiet"], stdin: Some("[]") },
    TestCase { args: &["--quiet", "--quiet"], stdin: Some("[]") },
    TestCase { args: &["--check", "--quiet"], stdin: Some("[]\n") },
    TestCase { args: &["--check", "--quiet"], stdin: Some("[\t]") },
    TestCase { args: &["--check", "--quiet", "--quiet"], stdin: Some("[]\n") },
    TestCase { args: &["--check", "--quiet", "--quiet"], stdin: Some("[\t]") },
    TestCase { args: &["--quiet"], stdin: Some("[") },
    TestCase { args: &["--quiet", "--quiet"], stdin: Some("[") },
    TestCase { args: &[".", "--exclude", ".", "--quiet"], stdin: None },
    TestCase {
        args:  &["--exclude", ".", "--quiet", "--quiet", "--", "."],
        stdin: None,
    },
    TestCase {
        args:  &["--check", "tests/inputs/changed.nix", "--quiet"],
        stdin: None,
    },
    TestCase {
        args:  &[
            "-c",
            "tests/inputs/changed.nix",
            "-q",
            "-e",
            "tests/changed.nix",
        ],
        stdin: None,
    },
    TestCase {
        args:  &["--check", "tests/inputs/changed.nix", "-qq"],
        stdin: None,
    },
    TestCase {
        args:  &["--check", "tests/inputs/unchanged.nix", "-q"],
        stdin: None,
    },
    TestCase {
        args:  &["--check", "tests/inputs/unchanged.nix", "-qq"],
        stdin: None,
    },
    TestCase {
        args:  &["--check", "tests/inputs/error.nix", "-q"],
        stdin: None,
    },
    TestCase {
        args:  &["--check", "tests/inputs/error.nix", "-qq"],
        stdin: None,
    },
    TestCase {
        args:  &[
            "--check",
            "tests/inputs/unchanged.nix",
            "--experimental-config",
            "../../alejandra.toml",
            "--threads",
            "1",
        ],
        stdin: None,
    },
    TestCase {
        args:  &[
            "--check",
            "tests/inputs/unchanged.nix",
            "--experimental-config",
            "tests/configs/empty_config.toml",
            "-t",
            "1",
        ],
        stdin: None,
    },
    TestCase {
        args:  &[
            "--check",
            "tests/inputs/unchanged.nix",
            "--experimental-config",
            "tests/configs/wrong_key.toml",
        ],
        stdin: None,
    },
];

#[test]
fn cases() {
    let should_update = std::env::var("UPDATE").is_ok();

    let output_path = PathBuf::new().join("tests").join("output.txt");

    let mut output_got = String::new();

    for case in CASES {
        output_got.push_str("===\n");
        output_got.push_str(&format!("args: {:?}\n", case.args));

        let mut child = Command::new("cargo")
            .args(["run", "--quiet", "--"])
            .args(case.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("command failed");

        if let Some(stdin) = case.stdin {
            output_got.push_str(&format!("stdin: {:?}\n", stdin));

            child
                .stdin
                .take()
                .unwrap()
                .write_all(stdin.as_bytes())
                .expect("unable to write to child stdin");
        }

        let output = child.wait_with_output().expect("child command failed");

        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8");
        if !stdout.is_empty() {
            output_got
                .push_str(&format!("stdout:\n{}\n", indent_and_clean(&stdout)));
        }

        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8");
        if !stderr.is_empty() {
            output_got
                .push_str(&format!("stderr:\n{}\n", indent_and_clean(&stderr)));
        }

        output_got
            .push_str(&format!("exit code: {:?}\n", output.status.code()));
    }

    if should_update {
        std::fs::File::create(&output_path)
            .unwrap()
            .write_all(output_got.as_bytes())
            .unwrap();
    }

    let output_expected = std::fs::read_to_string(&output_path).unwrap();

    assert_eq!(output_expected, output_got);
}

fn indent_and_clean(data: &str) -> String {
    data.lines().filter(|line| !line.starts_with(['👏', '🤟', '⭐'])).fold(
        String::new(),
        |mut output, line| {
            let _ = writeln!(output, "  {}", line);
            output
        },
    )
}
