mod app;
mod timer;
mod tui;
mod ui;

use std::time::Duration;

use app::{App, Message, State};
use color_eyre::eyre::Ok;
use crossterm::event::{self, Event, KeyCode};
use timer::run_timer;
use ui::ui;

fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut app = App::new();

    while app.current_screen != State::Done {
        terminal.draw(|f| ui(f, &app))?;

        let mut current_message = handle_event(&app)?;

        while current_message.is_some() {
            current_message = update(&mut app, current_message.unwrap());
        }
    }

    tui::restore_terminal()?;

    Ok(())
}

fn handle_event(_: &App) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }

    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Tab => Some(Message::Switch),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}

fn update(app: &mut App, msg: Message) -> Option<Message> {
    match msg {
        Message::Switch => match app.current_screen {
            State::Prompt => app.current_screen = State::Main,
            State::Main => app.current_screen = State::Prompt,
            _ => {}
        },
        Message::Quit => app.current_screen = State::Done,
    }
    None
}
