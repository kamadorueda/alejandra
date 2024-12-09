use std::fs::read_to_string;
use std::io::Read;

use alejandra::config::Config;
use clap::value_parser;
use clap::ArgAction;
use clap::Parser;
use futures::future::RemoteHandle;
use futures::stream::FuturesUnordered;
use futures::task::SpawnExt;

use crate::ads::random_ad;
use crate::verbosity::Verbosity;

/// The Uncompromising Nix Code Formatter.
#[derive(Debug, Parser)]
#[clap(
    name="Alejandra",

    after_help = concat!(
        "Alejandra will exit with status code:\n",
        "  1, if any error occurs.\n",
        "  2, if --check was used and any file requires formatting.\n",
        "  0, otherwise.",
    ),
    term_width = 80,
    version,
)]
struct CLIArgs {
    /// Files or directories, or a single "-" (or leave empty) to format stdin.
    #[clap(multiple_values = true)]
    include: Vec<String>,

    /// Files or directories to exclude from formatting.
    #[clap(long, short, multiple_occurrences = true)]
    exclude: Vec<String>,

    /// Check if the input is already formatted and disable writing in-place
    /// the modified content.
    #[clap(long, short)]
    check: bool,

    /// [Experimental] Path to a config file. If not provided, it'll default to
    /// `alejandra.toml` in the current directory. If not found, it'll use the
    /// default style.
    #[clap(long)]
    experimental_config: Option<String>,

    /// Number of formatting threads to spawn. Defaults to the number of
    /// physical CPUs.
    #[clap(long, short, value_parser = value_parser!(u8).range(1..))]
    threads: Option<u8>,

    /// Use once to hide informational messages,
    /// twice to hide error messages.
    #[clap(long, short, action = ArgAction::Count)]
    quiet: u8,
}

#[derive(Clone)]
struct FormattedPath {
    pub path:   String,
    pub status: alejandra::format::Status,
}

fn format_stdin(config: Config, verbosity: Verbosity) -> FormattedPath {
    let mut before = String::new();
    let path = "<anonymous file on stdin>".to_string();

    if verbosity.allows_info() {
        eprintln!("Formatting stdin.");
        eprintln!("Use --help to see all command line options.");
        eprintln!("use --quiet to suppress this and other messages.");
    }

    std::io::stdin()
        .read_to_string(&mut before)
        .expect("Unable to read stdin.");

    let (status, data) =
        alejandra::format::in_memory(path.clone(), before.clone(), config);

    print!("{data}");

    FormattedPath { path, status }
}

fn format_paths(
    paths: Vec<String>,
    config: Config,
    in_place: bool,
    threads: usize,
    verbosity: Verbosity,
) -> Vec<FormattedPath> {
    let paths_len = paths.len();

    if verbosity.allows_info() {
        eprintln!(
            "Checking style in {paths_len} file{} using {threads} thread{}.",
            if paths_len == 1 { "" } else { "s" },
            if threads == 1 { "" } else { "s" },
        );
        eprintln!();
    }

    let pool = futures::executor::ThreadPoolBuilder::new()
        .pool_size(threads)
        .create()
        .expect("Unable to instantiate a new thread pool.");

    let futures: FuturesUnordered<RemoteHandle<FormattedPath>> = paths
        .into_iter()
        .map(|path| {
            pool.spawn_with_handle(async move {
                let status =
                    alejandra::format::in_fs(path.clone(), config, in_place);

                if let alejandra::format::Status::Changed(changed) = status {
                    if changed && verbosity.allows_info() {
                        println!(
                            "{}: {path}",
                            if in_place {
                                "Formatted"
                            } else {
                                "Requires formatting"
                            },
                        );
                    }
                }

                FormattedPath { path: path.clone(), status }
            })
            .expect("Unable to spawn formatting task.")
        })
        .collect();

    futures::executor::block_on_stream(futures).collect()
}

pub fn main() -> ! {
    let args = CLIArgs::parse();

    let in_place = !args.check;

    let include: Vec<&str> =
        args.include.iter().map(String::as_str).collect::<Vec<&str>>();

    let threads =
        args.threads.map_or_else(num_cpus::get_physical, Into::<usize>::into);

    let verbosity = match args.quiet {
        0 => Verbosity::Everything,
        1 => Verbosity::NoInfo,
        _ => Verbosity::NoErrors,
    };

    let config = resolve_config(args.experimental_config.as_deref(), verbosity);

    let formatted_paths = match &include[..] {
        &[] | &["-"] => {
            vec![crate::cli::format_stdin(config, verbosity)]
        }
        include => {
            let paths = crate::find::nix_files(include, &args.exclude);

            crate::cli::format_paths(
                paths, config, in_place, threads, verbosity,
            )
        }
    };

    let errors = formatted_paths
        .iter()
        .filter(|formatted_path| {
            matches!(formatted_path.status, alejandra::format::Status::Error(_))
        })
        .count();

    if errors > 0 {
        if verbosity.allows_errors() {
            eprintln!();
            eprintln!(
                "Failed! {errors} error{} found at:",
                if errors == 1 { "" } else { "s" }
            );
            for formatted_path in formatted_paths {
                if let alejandra::format::Status::Error(error) =
                    formatted_path.status
                {
                    eprintln!("- {}: {error}", formatted_path.path);
                }
            }
        }

        std::process::exit(1);
    }

    let changed = formatted_paths
        .iter()
        .filter(|formatted_path| match formatted_path.status {
            alejandra::format::Status::Changed(changed) => changed,
            _ => false,
        })
        .count();

    if changed > 0 {
        if verbosity.allows_info() {
            eprintln!();
            eprintln!(
                "{}! {changed} file{} {}.",
                if in_place { "Success" } else { "Alert" },
                if changed == 1 { "" } else { "s" },
                match (changed == 1, in_place) {
                    (false, true) => "were formatted",
                    (false, false) => "require formatting",
                    (true, true) => "was formatted",
                    (true, false) => "requires formatting",
                }
            );

            if in_place {
                eprintln!();
                eprint!("{}", random_ad());
            }
        }

        std::process::exit(if in_place { 0 } else { 2 });
    }

    if verbosity.allows_info() {
        eprintln!();
        eprintln!(
            "Congratulations! Your code complies with the Alejandra style."
        );
        eprintln!();
        eprint!("{}", random_ad());
    }

    std::process::exit(0);
}

fn resolve_config(path: Option<&str>, verbosity: Verbosity) -> Config {
    let default_config_path = "alejandra.toml";

    // If no path was provided and the default config path exists, use it
    let path = path.or_else(|| {
        std::fs::exists(default_config_path)
            .unwrap_or(false)
            .then_some(default_config_path)
    });

    if let Some(path) = path {
        if verbosity.allows_info() {
            eprintln!("Using config from: {}", path);
        }

        let contents = read_to_string(path).unwrap_or_else(|error| {
            if verbosity.allows_errors() {
                eprintln!("Unable to read config: {}", error);
            }
            std::process::exit(1);
        });

        toml::from_str::<Config>(&contents).unwrap_or_else(|error| {
            if verbosity.allows_errors() {
                eprintln!("Errors found in config: {}", error);
            }
            std::process::exit(1);
        })
    } else {
        Config::default()
    }
}
