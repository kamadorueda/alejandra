//! Checks for the files that make Alejandra installable.

use std::path::PathBuf;

fn repo_file(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(name);

    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {path:?}: {error}"))
}

/// The lines of the `[section]` of a TOML file, without comments.
fn section(toml: &str, name: &str) -> Vec<String> {
    let header = format!("[{name}]");
    let mut lines = Vec::new();
    let mut inside = false;

    for line in toml.lines().map(str::trim) {
        if line.starts_with('[') {
            inside = line == header;
        } else if inside && !line.is_empty() && !line.starts_with('#') {
            lines.push(line.to_owned());
        }
    }

    lines
}

fn value(lines: &[String], key: &str) -> Option<String> {
    lines.iter().find_map(|line| {
        let (k, v) = line.split_once('=')?;

        (k.trim() == key).then(|| v.trim().trim_matches('"').to_owned())
    })
}

// https://github.com/kamadorueda/alejandra/issues/501
// `cargo install` ignores Cargo.lock, so a loose requirement on rnix
// (like `*`) picks a release that needs another rowan and does not compile.
// A requirement like `0.14.0` stays in 0.14.x, as does `=0.14.0`
#[test]
fn rnix_and_rowan_are_not_loose() {
    let toml = repo_file("src/alejandra/Cargo.toml");
    let dependencies = section(&toml, "dependencies");

    for name in ["rnix", "rowan"] {
        let line = dependencies
            .iter()
            .find(|line| line.starts_with(&format!("{name} ")))
            .unwrap_or_else(|| panic!("{name} is not a dependency"));

        let version = line
            .split("version = \"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .unwrap_or_else(|| panic!("{name} has no version: {line}"));

        let version = version.trim_start_matches(['=', '^']);
        let parts: Vec<&str> = version.split('.').collect();

        assert!(
            parts.len() == 3 && parts.iter().all(|p| p.parse::<u32>().is_ok()),
            "{name} must require a full version like 0.14.0, got: {line}"
        );
    }
}

// https://github.com/kamadorueda/alejandra/issues/501
// pre-commit runs `cargo install --path .` at the root of the repository
#[test]
fn root_is_a_package() {
    let toml = repo_file("Cargo.toml");

    assert!(
        !section(&toml, "package").is_empty(),
        "the root Cargo.toml must have a [package]"
    );
    assert!(
        !section(&toml, "workspace").is_empty(),
        "the root Cargo.toml must stay a workspace"
    );
}

// https://github.com/kamadorueda/alejandra/issues/501
#[test]
fn pre_commit_hook_entry_is_a_binary_of_the_root_package() {
    let hooks = repo_file(".pre-commit-hooks.yaml");
    let toml = repo_file("Cargo.toml");

    // The hook that installs with cargo
    let mut entry = None;
    let mut current_entry = None;
    let mut current_language = None;
    for line in hooks.lines().map(str::trim) {
        if line.starts_with("- id:") {
            if current_language.as_deref() == Some("rust") {
                entry = current_entry.take();
            }
            current_entry = None;
            current_language = None;
        } else if let Some(v) = line.strip_prefix("entry:") {
            current_entry = Some(v.trim().to_owned());
        } else if let Some(v) = line.strip_prefix("language:") {
            current_language = Some(v.trim().to_owned());
        }
    }
    if current_language.as_deref() == Some("rust") {
        entry = current_entry;
    }
    let entry = entry.expect("no hook with `language: rust`");

    // Every [[bin]] of the root package
    let mut bins = Vec::new();
    let mut inside = false;
    for line in toml.lines().map(str::trim) {
        if line.starts_with('[') {
            inside = line == "[[bin]]";
        } else if inside && line.starts_with("name") {
            bins.push(value(&[line.to_owned()], "name").unwrap());
        }
    }

    assert!(
        bins.contains(&entry),
        "the entry `{entry}` of the pre-commit hook is not a [[bin]] of the \
         root Cargo.toml: {bins:?}"
    );

    // And that binary points to a file that exists
    let bin_lines: Vec<String> = toml
        .lines()
        .map(|l| l.trim().to_owned())
        .skip_while(|l| l != "[[bin]]")
        .skip(1)
        .take_while(|l| !l.starts_with('['))
        .collect();
    let path = value(&bin_lines, "path").expect("[[bin]] without a path");
    assert!(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join(&path)
            .is_file(),
        "{path} does not exist"
    );
}
