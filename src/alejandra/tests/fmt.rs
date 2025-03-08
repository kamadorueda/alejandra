use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

use alejandra::config::Config;
use alejandra::config::Indentation;
use pretty_assertions::assert_eq;

#[test]
fn cases() {
    let should_update = std::env::var("UPDATE").is_ok();

    let configs = HashMap::from([
        ("default", Config::default()),
        ("indentation-tabs", Config { indentation: Indentation::Tabs }),
        ("indentation-fourspaces", Config { indentation: Indentation::FourSpaces }),
    ]);

    let cases_path = PathBuf::new().join("tests").join("cases");

    for (config_name, config) in configs {
        let config_cases_path = cases_path.join(config_name);

        let cases: Vec<String> = std::fs::read_dir(&config_cases_path)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();

        for case in cases {
            let case_path = config_cases_path.join(&case);

            let path_in = case_path.join("in.nix");
            let content_in = std::fs::read_to_string(&path_in).unwrap();

            let path_out = case_path.join("out.nix");
            let content_got = alejandra::format::in_memory(
                path_in.to_str().unwrap().to_owned(),
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

            let content_out = std::fs::read_to_string(&path_out).unwrap();

            assert_eq!(
                content_out, content_got,
                "Test case `{:?}` failed",
                case_path
            );
        }
    }
}
