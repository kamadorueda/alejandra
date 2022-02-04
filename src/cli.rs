pub fn parse(args: Vec<String>) -> clap::ArgMatches {
    clap::App::new("Alejandra")
        .about("The Uncompromising Nix Code Formatter.")
        .arg(
            clap::Arg::new("debug")
                .help("Enable debug mode.")
                .long("debug")
                .short('d')
                .takes_value(false),
        )
        .arg(
            clap::Arg::new("max-width")
                .default_value("80")
                .help("How many characters per line to allow.")
                .long("max-width")
                .takes_value(true)
                .value_name("CHARS"),
        )
        .arg(
            clap::Arg::new("paths")
                .help("Files or directories, or none to format stdin.")
                .multiple_values(true),
        )
        .term_width(80)
        .get_matches_from(args)
}
