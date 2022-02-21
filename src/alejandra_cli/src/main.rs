fn main() -> std::io::Result<()> {
    let matches = alejandra_cli::cli::parse(std::env::args().collect());

    let config = alejandra_engine::config::Config::default();

    match matches.values_of("include") {
        Some(include) => {
            let include = include.collect();
            let exclude = match matches.values_of("exclude") {
                Some(exclude) => exclude.collect(),
                None => vec![],
            };

            let paths: Vec<String> =
                alejandra_cli::find::nix_files(include, exclude);

            if atty::is(atty::Stream::Stderr)
                && atty::is(atty::Stream::Stdin)
                && atty::is(atty::Stream::Stdout)
            {
                alejandra_cli::cli::tui(config, paths)?;
            } else {
                alejandra_cli::cli::simple(config, paths)?;
            }
        }
        None => {
            alejandra_cli::cli::stdin(config)?;
        }
    }

    std::process::exit(0);
}
