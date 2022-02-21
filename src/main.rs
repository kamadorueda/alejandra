fn main() -> std::io::Result<()> {
    let matches = alejandra::cli::parse(std::env::args().collect());

    let config = alejandra::config::Config::new();

    match matches.values_of("paths") {
        Some(paths) => {
            let paths: Vec<String> =
                alejandra::find::nix_files(paths.collect());

            if atty::is(atty::Stream::Stderr)
                && atty::is(atty::Stream::Stdin)
                && atty::is(atty::Stream::Stdout)
            {
                alejandra::cli::tui(config, paths)?;
            } else {
                alejandra::cli::simple(config, paths)?;
            }
        }
        None => {
            alejandra::cli::stdin(config)?;
        }
    }

    std::process::exit(0);
}
