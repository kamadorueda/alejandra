pub fn parse(args: Vec<String>) -> clap::ArgMatches {
    clap::Command::new("Alejandra")
        .about("The Uncompromising Nix Code Formatter.")
        .version(crate::version::VERSION)
        .arg(
            clap::Arg::new("debug")
                .help("Enable debug mode.")
                .long("debug")
                .short('d')
                .takes_value(false),
        )
        .arg(
            clap::Arg::new("paths")
                .help("Files or directories, or none to format stdin.")
                .multiple_values(true),
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
            Shaped with love by:
            - Kevin Amado ~ @kamadorueda on GitHub, matrix.org and Gmail.
            - Thomas Bereknyei ~ @tomberek on GitHub and matrix.org.
            - David Arnold ~ @blaggacao on GitHub and matrix.org.
            - Vincent Ambo ~ @tazjin on GitHub.
            - Mr Hedgehog ~ @ModdedGamers on GitHub.
            "
        ))
        .get_matches_from(args)
}

pub fn stdin(config: crate::config::Config) -> std::io::Result<()> {
    use std::io::Read;

    eprintln!("Formatting stdin, run with --help to see all options.");
    let mut stdin = String::new();
    std::io::stdin().read_to_string(&mut stdin).unwrap();
    print!("{}", crate::format::string(&config, "stdin".to_string(), stdin));

    Ok(())
}

pub fn simple(
    config: crate::config::Config,
    paths: Vec<String>,
) -> std::io::Result<()> {
    use rayon::prelude::*;

    eprintln!("Formatting {} files.", paths.len());

    let (results, errors): (Vec<_>, Vec<_>) = paths
        .par_iter()
        .map(|path| -> std::io::Result<bool> {
            crate::format::file(&config, path.to_string()).map(|changed| {
                if changed {
                    eprintln!("Formatted: {}", &path);
                }
                changed
            })
        })
        .partition(Result::is_ok);

    eprintln!(
        "Changed: {}",
        results.into_iter().map(Result::unwrap).filter(|&x| x).count(),
    );
    eprintln!("Errors: {}", errors.len(),);

    Ok(())
}

pub fn tui(
    config: crate::config::Config,
    paths: Vec<String>,
) -> std::io::Result<()> {
    use rayon::prelude::*;
    use termion::{input::TermRead, raw::IntoRawMode};

    struct FormattedPath {
        path:   String,
        result: std::io::Result<bool>,
    }

    enum Event {
        FormattedPath(FormattedPath),
        FormattingFinished,
        Input(termion::event::Key),
        Tick,
    }

    let paths_to_format = paths.len();

    let stdout = std::io::stderr().into_raw_mode()?;
    // let stdout = termion::screen::AlternateScreen::from(stdout);
    let backend = tui::backend::TermionBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;

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
    let sender_finished = sender.clone();
    std::thread::spawn(move || {
        paths.into_par_iter().for_each_with(sender_paths, |sender, path| {
            let result = crate::format::file(&config, path.clone());

            if let Err(error) = sender
                .send(Event::FormattedPath(FormattedPath { path, result }))
            {
                eprintln!("{}", error);
            }
        });

        if let Err(error) = sender_finished.send(Event::FormattingFinished) {
            eprintln!("{}", error);
        }
    });

    terminal.clear()?;

    let mut finished = false;
    let mut paths_with_errors: usize = 0;
    let mut paths_changed: usize = 0;
    let mut paths_unchanged: usize = 0;
    let mut formatted_paths = std::collections::LinkedList::new();

    while !finished {
        loop {
            match receiver.try_recv() {
                Ok(event) => match event {
                    Event::FormattedPath(formatted_path) => {
                        match formatted_path.result {
                            Ok(changed) => {
                                if changed {
                                    paths_changed += 1;
                                } else {
                                    paths_unchanged += 1;
                                }
                            }
                            Err(_) => {
                                paths_with_errors += 1;
                            }
                        };

                        formatted_paths.push_back(formatted_path);
                        if formatted_paths.len() > 8 {
                            formatted_paths.pop_front();
                        }
                    }
                    Event::FormattingFinished => {
                        finished = true;
                    }
                    Event::Input(key) => {
                        match key {
                            termion::event::Key::Ctrl('c') => {
                                return Err(
                                    std::io::ErrorKind::Interrupted.into()
                                );
                            }
                            _ => {}
                        };
                    }
                    Event::Tick => {
                        break;
                    }
                },
                Err(_) => {}
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
                    tui::text::Span::raw(crate::version::VERSION),
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
                .percent(
                    (100 * (paths_changed
                        + paths_unchanged
                        + paths_with_errors)
                        / paths_to_format) as u16,
                )
                .style(
                    tui::style::Style::default()
                        .bg(tui::style::Color::Black)
                        .fg(tui::style::Color::White),
                );
            let logger = tui::widgets::Paragraph::new(
                formatted_paths
                    .iter()
                    .map(|formatted_path| {
                        tui::text::Spans::from(vec![
                            match &formatted_path.result {
                                Ok(changed) => tui::text::Span::styled(
                                    if *changed {
                                        "CHANGED   "
                                    } else {
                                        "UNCHANGED "
                                    },
                                    tui::style::Style::default()
                                        .fg(tui::style::Color::Green),
                                ),
                                Err(_) => tui::text::Span::styled(
                                    "ERROR     ",
                                    tui::style::Style::default()
                                        .fg(tui::style::Color::Red),
                                ),
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

    Ok(())
}
