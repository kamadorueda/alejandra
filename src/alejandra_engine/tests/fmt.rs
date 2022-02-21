use std::io::Write;

#[test]
fn cases() {
    let should_update = std::env::var("UPDATE").is_ok();

    let config = alejandra_engine::config::Config::default();

    let cases: std::collections::HashSet<String> =
        std::fs::read_dir("tests/cases")
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();

    for case in cases {
        let path_in = format!("tests/cases/{}/in", case);
        let path_out = format!("tests/cases/{}/out", case);
        let content_in = std::fs::read_to_string(path_in.clone()).unwrap();
        let content_got = alejandra_engine::format::string_or_passthrough(
            &config,
            path_in,
            content_in.clone(),
        );

        if should_update {
            std::fs::File::create(&path_out)
                .unwrap()
                .write_all(content_got.as_bytes())
                .unwrap();
        }

        let content_out = std::fs::read_to_string(path_out.clone()).unwrap();

        assert!(content_got == content_out);
    }
}
