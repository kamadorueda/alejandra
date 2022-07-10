use clap::Parser;

#[derive(Clone)]
pub(crate) struct FormattedPath {
    pub path:   String,
    pub status: alejandra::format::Status,
}

const AFTER_HELP: &str = indoc::indoc! {"
    The program will exit with status code:
      1, if any error occurs.
      2, if --check was used and any file was changed.
      0, otherwise.

    Your star and feedback is very much appreciated!
      https://github.com/kamadorueda/alejandra
    "
};

/// The Uncompromising Nix Code Formatter
#[derive(Debug, Parser)]
#[clap(version, after_help = AFTER_HELP, term_width = 80)]
struct Args {
    /// Files or directories, or none to format stdin
    #[clap(multiple_values = true)]
    include: Vec<String>,

    /// Files or directories to exclude from formatting
    #[clap(long, short, multiple_occurrences = true)]
    exclude: Vec<String>,

    /// Check if the input is already formatted and disable writing in-place
    /// the modified content
    #[clap(long, short)]
    check: bool,

    /// Number of formatting threads to spawn. Defaults to the number of
    /// logical CPUs.
    #[clap(long, short)]
    threads: Option<usize>,

    /// Hide the details, only show error messages.
    #[clap(long, short)]
    quiet: bool,
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
        alejandra::format::in_memory(path.clone(), before.clone());

    print!("{}", data);

    FormattedPath { path, status }
}

pub(crate) fn simple(
    paths: Vec<String>,
    in_place: bool,
    quiet: bool,
) -> Vec<FormattedPath> {
    use rayon::prelude::*;

    if !quiet {
        eprintln!("Processing: {} files", paths.len());
    }

    paths
        .par_iter()
        .map(|path| {
            let status = alejandra::format::in_fs(path.clone(), in_place);

            if let alejandra::format::Status::Changed(changed) = status {
                if changed && !quiet {
                    if in_place {
                        eprintln!("Changed: {}", &path);
                    } else {
                        eprintln!("Would be changed: {}", &path);
                    }
                }
            }

            FormattedPath { path: path.clone(), status }
        })
        .collect()
}

pub(crate) fn tui(
    paths: Vec<String>,
    in_place: bool,
) -> std::io::Result<Vec<FormattedPath>> {
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
            let status = alejandra::format::in_fs(path.clone(), in_place);

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
                            alejandra::format::Status::Changed(changed) => {
                                if *changed {
                                    paths_changed += 1;
                                } else {
                                    paths_unchanged += 1;
                                }
                            }
                            alejandra::format::Status::Error(_) => {
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
                    tui::text::Span::raw(alejandra::version::VERSION),
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
                            " Formatting ({} {}, {} unchanged, {} errors) ",
                            paths_changed,
                            if in_place {
                                "changed"
                            } else {
                                "would be changed"
                            },
                            paths_unchanged,
                            paths_with_errors
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
                                alejandra::format::Status::Changed(changed) => {
                                    tui::text::Span::styled(
                                        if changed {
                                            if in_place {
                                                "CHANGED "
                                            } else {
                                                "WOULD BE CHANGED "
                                            }
                                        } else {
                                            "OK "
                                        },
                                        tui::style::Style::default()
                                            .fg(tui::style::Color::Green),
                                    )
                                }
                                alejandra::format::Status::Error(_) => {
                                    tui::text::Span::styled(
                                        "ERROR ",
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
    let args = Args::parse();

    let in_place = !args.check;
    let threads = args.threads.unwrap_or(0);

    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();

    let formatted_paths = match &args.include[..] {
        &[] => {
            vec![crate::cli::stdin(args.quiet)]
        }
        include => {
            let paths = crate::find::nix_files(include, &args.exclude);

            if !args.quiet
                && atty::is(atty::Stream::Stderr)
                && atty::is(atty::Stream::Stdin)
                && atty::is(atty::Stream::Stdout)
            {
                crate::cli::tui(paths, in_place)?
            } else {
                crate::cli::simple(paths, in_place, args.quiet)
            }
        }
    };

    let errors = formatted_paths
        .iter()
        .filter(|formatted_path| {
            matches!(formatted_path.status, alejandra::format::Status::Error(_))
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
            if let alejandra::format::Status::Error(error) =
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
            alejandra::format::Status::Changed(changed) => changed,
            _ => false,
        })
        .count();

    if changed > 0 {
        if !args.quiet {
            eprintln!();
            eprintln!(
                "Success! {} file{} {}",
                changed,
                if changed >= 2 { "s" } else { "" },
                if in_place {
                    if changed >= 2 { "were changed" } else { "was changed" }
                } else {
                    "would be changed"
                }
            );
        }

        std::process::exit(if in_place { 0 } else { 2 });
    }

    if !args.quiet {
        eprintln!();
        eprintln!("Success! Your code complies the Alejandra style");
    }

    std::process::exit(0);
}
