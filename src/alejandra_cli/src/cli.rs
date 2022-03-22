#[derive(Clone)]
pub(crate) struct FormattedPath {
    pub path:   String,
    pub status: alejandra_engine::format::Status,
}

pub(crate) fn parse(args: Vec<String>) -> clap::ArgMatches {
    clap::Command::new("Alejandra")
        .about("The Uncompromising Nix Code Formatter.")
        .version(alejandra_engine::version::VERSION)
        .arg(
            clap::Arg::new("include")
                .help("Files or directories, or none to format stdin.")
                .multiple_values(true),
        )
        .arg(
            clap::Arg::new("exclude")
                .short('e')
                .help("Files or directories to exclude from formatting.")
                .long("exclude")
                .multiple_occurrences(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("check")
                .help("Check if the input is already formatted.")
                .long("--check")
                .short('c'),
        )
        .arg(
            clap::Arg::new("threads")
                .default_value("0")
                .help(
                    "Number of formatting threads to spawn. Defaults to the \
                     number of logical CPUs.",
                )
                .long("--threads")
                .short('t')
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("quiet")
                .help("Hide the details, only show error messages.")
                .long("--quiet")
                .short('q'),
        )
        .term_width(80)
        .after_help(
            #[cfg_attr(rustfmt, rustfmt_skip)]
            indoc::indoc!(
                // Let's just use the same sorting as on GitHub
                //
                // There are some non-code contributors,
                // I'm sorting those subjectively.
                // I've tried to be as just as possible.
                //
                // Feel free to add here your contact/blog/links if you want
                "
                The program will exit with status code:
                  1, if any error occurs.
                  2, if --check was used and any file was changed.
                  0, otherwise.

                Shaped with love by:
                  Kevin Amado ~ @kamadorueda on GitHub, matrix.org and Gmail.
                  Thomas Bereknyei ~ @tomberek on GitHub and matrix.org.
                  Piegames ~ @piegamesde on GitHub.
                  Joachim Ernst ~ @0x4A6F on GitHub.
                  David Arnold ~ @blaggacao on GitHub and matrix.org.
                  David Hauer ~ @DavHau on GitHub.
                  Fabian Möller ~ @B4dM4n on GitHub.
                  Rok Garbas ~ @garbas on GitHub.
                  Yorick van Pelt ~ @yorickvP on GitHub.
                  Jörg Thalheim ~ @Mic92 on GitHub.
                  Vincent Ambo ~ @tazjin on GitHub.
                  Mr Hedgehog ~ @ModdedGamers on GitHub.
                  Tristan Maat ~ @TLATER on GitHub.
                  Norbert Melzer ~ @NobbZ on GitHub.
                  Patrick Stevens ~ @Smaug123 on GitHub.
                  Florian Finkernagel ~ @TyberiusPrime on GitHub.

                Your star and feedback is very much appreciated!
                  https://github.com/kamadorueda/alejandra
                "
            ),
        )
        .get_matches_from(args)
}

pub(crate) fn stdin(quiet: bool) -> FormattedPath {
    use std::io::Read;

    let mut before = String::new();
    let path = "<anonymous file on stdin>".to_string();

    if !quiet {
        eprintln!("Formatting stdin, run with --help to see all options.");
    }

    std::io::stdin().read_to_string(&mut before).unwrap();

    let (status, data) =
        alejandra_engine::format::in_memory(path.clone(), before.clone());

    print!("{}", data);

    FormattedPath { path, status }
}

pub(crate) fn simple(paths: Vec<String>, quiet: bool) -> Vec<FormattedPath> {
    use rayon::prelude::*;

    if !quiet {
        eprintln!("Formatting: {} files", paths.len());
    }

    paths
        .par_iter()
        .map(|path| {
            let status = alejandra_engine::format::in_place(path.clone());

            if let alejandra_engine::format::Status::Changed(changed) = status {
                if changed && !quiet {
                    eprintln!("Changed: {}", &path);
                }
            }

            FormattedPath { path: path.clone(), status }
        })
        .collect()
}

pub(crate) fn tui(paths: Vec<String>) -> std::io::Result<Vec<FormattedPath>> {
    use rayon::prelude::*;
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;

    enum Event {
        FormattedPath(FormattedPath),
        FormattingFinished,
        Input(termion::event::Key),
        Tick,
    }

    let paths_to_format = paths.len();
    let mut formatted_paths = std::collections::LinkedList::new();

    let stdout = std::io::stderr().into_raw_mode()?;
    let backend = tui::backend::TermionBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;
    terminal.clear()?;

    let (sender, receiver) = std::sync::mpsc::channel();

    // Listen to user input
    let sender_keys = sender.clone();
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        for key in stdin.keys().flatten() {
            if let Err(error) = sender_keys.send(Event::Input(key)) {
                eprintln!("{}", error);
                return;
            }
        }
    });

    // Listen to the clock
    let sender_clock = sender.clone();
    std::thread::spawn(move || {
        loop {
            if let Err(error) = sender_clock.send(Event::Tick) {
                eprintln!("{}", error);
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
    });

    // Listen to the processed items
    let sender_paths = sender.clone();
    let sender_finished = sender;
    std::thread::spawn(move || {
        paths.into_par_iter().for_each_with(sender_paths, |sender, path| {
            let status = alejandra_engine::format::in_place(path.clone());

            if let Err(error) = sender
                .send(Event::FormattedPath(FormattedPath { path, status }))
            {
                eprintln!("{}", error);
            }
        });

        if let Err(error) = sender_finished.send(Event::FormattingFinished) {
            eprintln!("{}", error);
        }
    });

    let mut finished = false;
    let mut paths_with_errors: usize = 0;
    let mut paths_changed: usize = 0;
    let mut paths_unchanged: usize = 0;

    while !finished {
        loop {
            if let Ok(event) = receiver.try_recv() {
                match event {
                    Event::FormattedPath(formatted_path) => {
                        match &formatted_path.status {
                            alejandra_engine::format::Status::Changed(
                                changed,
                            ) => {
                                if *changed {
                                    paths_changed += 1;
                                } else {
                                    paths_unchanged += 1;
                                }
                            }
                            alejandra_engine::format::Status::Error(_) => {
                                paths_with_errors += 1;
                            }
                        };

                        formatted_paths.push_back(formatted_path);
                    }
                    Event::FormattingFinished => {
                        finished = true;
                    }
                    Event::Input(key) => {
                        if let termion::event::Key::Ctrl('c') = key {
                            return Err(std::io::ErrorKind::Interrupted.into());
                        }
                    }
                    Event::Tick => {
                        break;
                    }
                }
            }
        }

        terminal.draw(|frame| {
            let sizes = tui::layout::Layout::default()
                .constraints([
                    tui::layout::Constraint::Length(3),
                    tui::layout::Constraint::Length(3),
                    tui::layout::Constraint::Max(8),
                    tui::layout::Constraint::Length(0),
                ])
                .split(frame.size());
            let size = tui::layout::Rect::new(0, 0, 0, 0)
                .union(sizes[0])
                .union(sizes[1])
                .union(sizes[2]);

            let header = tui::widgets::Paragraph::new(vec![
                tui::text::Spans::from(vec![
                    tui::text::Span::styled(
                        "Alejandra",
                        tui::style::Style::default()
                            .fg(tui::style::Color::Green),
                    ),
                    tui::text::Span::raw(" "),
                    tui::text::Span::raw(alejandra_engine::version::VERSION),
                ]),
                tui::text::Spans::from(vec![tui::text::Span::raw(
                    "The Uncompromising Nix Code Formatter",
                )]),
            ])
            .alignment(tui::layout::Alignment::Center)
            .style(
                tui::style::Style::default()
                    .bg(tui::style::Color::Black)
                    .fg(tui::style::Color::White),
            );

            let progress = tui::widgets::Gauge::default()
                .block(
                    tui::widgets::Block::default()
                        .borders(tui::widgets::Borders::ALL)
                        .title(format!(
                            " Formatting ({} changed, {} unchanged, {} \
                             errors) ",
                            paths_changed, paths_unchanged, paths_with_errors
                        )),
                )
                .gauge_style(
                    tui::style::Style::default()
                        .fg(tui::style::Color::Green)
                        .bg(tui::style::Color::Black)
                        .add_modifier(tui::style::Modifier::ITALIC),
                )
                .percent(if paths_to_format == 0 {
                    100
                } else {
                    100 * (paths_changed + paths_unchanged + paths_with_errors)
                        / paths_to_format
                } as u16)
                .style(
                    tui::style::Style::default()
                        .bg(tui::style::Color::Black)
                        .fg(tui::style::Color::White),
                );
            let logger = tui::widgets::Paragraph::new(
                formatted_paths
                    .iter()
                    .rev()
                    .take(8)
                    .map(|formatted_path| {
                        tui::text::Spans::from(vec![
                            match formatted_path.status {
                                alejandra_engine::format::Status::Changed(
                                    changed,
                                ) => tui::text::Span::styled(
                                    if changed {
                                        "CHANGED "
                                    } else {
                                        "OK      "
                                    },
                                    tui::style::Style::default()
                                        .fg(tui::style::Color::Green),
                                ),
                                alejandra_engine::format::Status::Error(_) => {
                                    tui::text::Span::styled(
                                        "ERROR   ",
                                        tui::style::Style::default()
                                            .fg(tui::style::Color::Red),
                                    )
                                }
                            },
                            tui::text::Span::raw(formatted_path.path.clone()),
                        ])
                    })
                    .collect::<Vec<tui::text::Spans>>(),
            )
            .style(
                tui::style::Style::default()
                    .bg(tui::style::Color::Black)
                    .fg(tui::style::Color::White),
            );

            frame.render_widget(header, sizes[0]);
            frame.render_widget(progress, sizes[1]);
            frame.render_widget(logger, sizes[2]);
            frame.set_cursor(size.width, size.height);
        })?;
    }

    Ok(formatted_paths.iter().cloned().collect())
}

pub fn main() -> std::io::Result<()> {
    let matches = crate::cli::parse(std::env::args().collect());

    let check = matches.is_present("check");
    let threads = matches.value_of("threads").unwrap();
    let threads: usize = threads.parse().unwrap();
    let quiet = matches.is_present("quiet");

    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();

    let formatted_paths = match matches.values_of("include") {
        Some(include) => {
            let include = include.collect();
            let exclude = match matches.values_of("exclude") {
                Some(exclude) => exclude.collect(),
                None => vec![],
            };

            let paths: Vec<String> = crate::find::nix_files(include, exclude);

            if !quiet
                && atty::is(atty::Stream::Stderr)
                && atty::is(atty::Stream::Stdin)
                && atty::is(atty::Stream::Stdout)
            {
                crate::cli::tui(paths)?
            } else {
                crate::cli::simple(paths, quiet)
            }
        }
        None => vec![crate::cli::stdin(quiet)],
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
            if errors >= 2 { "s" } else { "" }
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
        if !quiet {
            eprintln!();
            eprintln!(
                "Success! {} file{} {} changed",
                changed,
                if changed >= 2 { "s" } else { "" },
                if changed >= 2 { "were" } else { "was" },
            );
        }
        if check {
            std::process::exit(2);
        } else {
            std::process::exit(0);
        }
    }

    if !quiet {
        eprintln!();
        eprintln!("Success! Your code complies the Alejandra style");
    }
    std::process::exit(0);
}
