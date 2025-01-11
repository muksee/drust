use std::io::{
    self,
    stdout,
};

use crossterm::{
    event::{
        self,
        Event,
        KeyCode,
    },
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    style::Stylize,
    widgets::*,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press
                && key.code == KeyCode::Char('q')
            {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn ui(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ],
    )
    .split(frame.size());

    let span1 = Span::raw("Hello ");
    let span2 = Span::styled(
        "World",
        Style::new()
            .fg(Color::Green)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    let span3 = "!"
        .red()
        .on_light_yellow()
        .italic();
    let line: Line<'_> = Line::from(vec![span1, span2, span3]);
    let text: Text = Text::from(line);

    frame.render_widget(Paragraph::new(text), main_layout[0]);
    frame.render_widget(
        Paragraph::new(
            "Hello World"
                .red()
                .on_white()
                .bold(),
        ),
        main_layout[1],
    );
    frame.render_widget(
        Paragraph::new("Hello World!").style(
            Style::new()
                .red()
                .on_white(),
        ),
        main_layout[2],
    );
    frame.render_widget(
        Paragraph::new("Hello World!")
            .blue()
            .on_yellow(),
        main_layout[3],
    );
}
