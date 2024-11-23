use std::collections::HashMap;
use std::io::Write;

use alejandra::config::Config;
use alejandra::config::Indentation;
use pretty_assertions::assert_eq;

#[test]
fn cases() {
    let should_update = std::env::var("UPDATE").is_ok();

    let configs = HashMap::from([
        ("default", Config::default()),
        ("indentation-tabs", Config {
            indentation: Indentation::Tabs,
            ..Default::default()
        }),
    ]);

    for (config_name, config) in configs {
        let cases: Vec<String> =
            std::fs::read_dir(format!("tests/cases/{}", config_name))
                .unwrap()
                .map(|entry| entry.unwrap().file_name().into_string().unwrap())
                .collect();

        for case in cases {
            let path_in =
                format!("tests/cases/{}/{}/in.nix", config_name, case);
            let content_in = std::fs::read_to_string(&path_in).unwrap();

            let path_out =
                format!("tests/cases/{}/{}/out.nix", config_name, case);
            let content_got = alejandra::format::in_memory(
                path_in.clone(),
                content_in.clone(),
                config,
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
                "Test case `{}/{}` failed; see \
                 `src/alejandra/tests/cases/{}/{}/`",
                config_name, case, config_name, case,
            );
        }
    }
}
