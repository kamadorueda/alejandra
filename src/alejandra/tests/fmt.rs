use std::io::Write;

use pretty_assertions::assert_eq;

#[test]
fn cases() {
    let should_update = std::env::var("UPDATE").is_ok();

    let cases: std::collections::HashSet<String> =
        std::fs::read_dir("tests/cases")
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();

    for case in cases {
        let path_in = format!("tests/cases/{}/in.nix", case);
        let path_out = format!("tests/cases/{}/out.nix", case);
        let content_in = std::fs::read_to_string(path_in.clone()).unwrap();
        let content_got =
            alejandra::format::in_memory(path_in, content_in.clone(), " ".repeat(2)).1;

        if should_update {
            std::fs::File::create(&path_out)
                .unwrap()
                .write_all(content_got.as_bytes())
                .unwrap();
        }

        let content_out = std::fs::read_to_string(path_out.clone()).unwrap();

        assert_eq!(
            content_out, content_got,
            "Test case `{case}` failed; see \
             `src/alejandra/tests/cases/{case}/`"
        );
    }
}

#[test]
fn indent() {
    let should_update = std::env::var("UPDATE").is_ok();

    let cases: std::collections::HashSet<String> =
        std::fs::read_dir("tests/indent")
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();

    for case in cases {
        let path_in = format!("tests/indent/{}/in.nix", case);
        let path_out = format!("tests/indent/{}/out.nix", case);
        let content_in = std::fs::read_to_string(path_in.clone()).unwrap();
        let content_got =
            alejandra::format::in_memory(path_in, content_in.clone(), "\t".to_string()).1;

        if should_update {
            std::fs::File::create(&path_out)
                .unwrap()
                .write_all(content_got.as_bytes())
                .unwrap();
        }

        let content_out = std::fs::read_to_string(path_out.clone()).unwrap();

        assert_eq!(
            content_out, content_got,
            "Test case `{case}` failed; see \
             `src/alejandra/tests/indent/{case}/`"
        );
    }
}
