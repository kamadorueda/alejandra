use rayon::prelude::*;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let matches = alejandra::cli::parse(std::env::args().collect());

    let debug: bool = matches.is_present("debug");
    let config = alejandra::config::Config::new().with_debug(debug);

    match matches.values_of("paths") {
        Some(paths) => {
            let paths: Vec<String> =
                alejandra::find::nix_files(paths.collect());

            eprintln!("Formatting {} files.", paths.len());

            for result in paths
                .par_iter()
                .map(|path| -> std::io::Result<()> {
                    eprintln!("Formatting: {}", &path);
                    alejandra::format::file(&config, path.to_string())?;
                    Ok(())
                })
                .collect::<Vec<std::io::Result<()>>>()
            {
                result?;
            }
        }
        None => {
            eprintln!("Formatting stdin.");
            let mut stdin = String::new();
            std::io::stdin().read_to_string(&mut stdin).unwrap();
            print!(
                "{}",
                alejandra::format::string(&config, "stdin".to_string(), stdin)
            );
        }
    }

    std::process::exit(0);
}
