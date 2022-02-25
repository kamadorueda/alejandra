#[derive(Clone)]
pub struct FormattedPath {
    pub path:   String,
    pub status: alejandra_engine::format::Status,
}

pub fn parse(args: Vec<String>) -> clap::ArgMatches {
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
                .long("--check"),
        )
        .term_width(80)
        .after_help(indoc::indoc!(
            // Let's just use the same sorting as on GitHub
            //
            // There are some non-code contributors,
            // I'm sorting those subjectively
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
              David Arnold ~ @blaggacao on GitHub and matrix.org.
              Vincent Ambo ~ @tazjin on GitHub.
              Mr Hedgehog ~ @ModdedGamers on GitHub.
            "
        ))
        .get_matches_from(args)
}

pub fn stdin() -> FormattedPath {
    use std::io::Read;

    let mut before = String::new();
    let path = "<anonymous file on stdin>".to_string();

    eprintln!("Formatting stdin, run with --help to see all options.");

    std::io::stdin().read_to_string(&mut before).unwrap();

    let (status, data) =
        alejandra_engine::format::in_memory(path.clone(), before.clone());

    print!("{}", data);

    FormattedPath { path, status }
}

pub fn simple(paths: Vec<String>) -> Vec<FormattedPath> {
    use rayon::prelude::*;

    eprintln!("Formatting: {} files", paths.len());

    paths
        .par_iter()
        .map(|path| {
            let status = alejandra_engine::format::in_place(path.clone());

            if let alejandra_engine::format::Status::Changed(changed) = status {
                if changed {
                    eprintln!("Changed: {}", &path);
                }
            }

            FormattedPath { path: path.clone(), status }
        })
        .collect()
}

pub fn tui(paths: Vec<String>) -> std::io::Result<Vec<FormattedPath>> {
    use rayon::prelude::*;
    use termion::{input::TermRead, raw::IntoRawMode};

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
