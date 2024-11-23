use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;

use alejandra::config::Config;
use pretty_assertions::assert_eq;

#[test]
fn cases() {
    let should_update = std::env::var("UPDATE").is_ok();

    let cases: HashSet<String> = std::fs::read_dir("tests/cases")
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();

    let configs = HashMap::from([("", Config::default())]);

    for case in cases {
        let path_in = format!("tests/cases/{}/in.nix", case);
        let content_in = std::fs::read_to_string(path_in.clone()).unwrap();

        for (config_name, config) in &configs {
            let path_out =
                format!("tests/cases/{}/out{}.nix", case, config_name);
            let content_got = alejandra::format::in_memory(
                path_in.clone(),
                content_in.clone(),
                config.clone(),
            )
            .1;

            if should_update {
                std::fs::File::create(&path_out)
                    .unwrap()
                    .write_all(content_got.as_bytes())
                    .unwrap();
            }

            let content_out =
                std::fs::read_to_string(path_out.clone()).unwrap();

            assert_eq!(
                content_out, content_got,
                "Test case `{case}` failed; see \
                 `src/alejandra/tests/cases/{case}/`"
            );
        }
    }
}
