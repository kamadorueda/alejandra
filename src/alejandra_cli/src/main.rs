fn main() -> std::io::Result<()> {
    let matches = alejandra_cli::cli::parse(std::env::args().collect());

    let check = matches.is_present("check");

    let formatted_paths = match matches.values_of("include") {
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
                alejandra_cli::cli::tui(paths)?
            } else {
                alejandra_cli::cli::simple(paths)
            }
        }
        None => vec![alejandra_cli::cli::stdin()],
    };

    let errors = formatted_paths
        .iter()
        .filter(|formatted_path| {
            matches!(
                formatted_path.status,
                alejandra_engine::format::Status::Error(_)
            )
        })
        .count();

    if errors > 0 {
        eprintln!();
        eprintln!(
            "Failed! We encountered {} error{} at:",
            errors,
            if errors > 0 { "s" } else { "" }
        );
        for formatted_path in formatted_paths {
            if let alejandra_engine::format::Status::Error(error) =
                formatted_path.status
            {
                eprintln!("  {}: {}", formatted_path.path, &error);
            }
        }
        std::process::exit(1);
    }

    let changed = formatted_paths
        .iter()
        .filter(|formatted_path| match formatted_path.status {
            alejandra_engine::format::Status::Changed(changed) => changed,
            _ => false,
        })
        .count();

    if changed > 0 {
        eprintln!();
        eprintln!(
            "Success! {} file{} {} changed",
            changed,
            if changed > 0 { "s" } else { "" },
            if changed > 0 { "were" } else { "was" },
        );
        if check {
            std::process::exit(2);
        } else {
            std::process::exit(0);
        }
    }

    eprintln!();
    eprintln!("Success! Your code complies the Alejandra style");
    std::process::exit(0);
}
