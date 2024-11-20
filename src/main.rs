mod app;
mod timer;
mod tui;
mod ui;

use std::{sync::mpsc, thread, time::Duration};

use app::{App, Message, State};
use chrono::Local;
use crossterm::event::{self, Event, KeyCode};
use timer::run_timer;
use ui::ui;

fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut app = App::new();

    let (tx, rx) = mpsc::channel::<Message>();

    thread::spawn(move || {
        let _ = run_timer(tx);
    });

    while app.current_screen != State::Done {
        terminal.draw(|f| ui(f, &app))?;

        let mut current_message = handle_event(&mut app)?;

        while current_message.is_some() {
            current_message = update(&mut app, current_message.unwrap());
        }

        if let Ok(msg) = rx.recv_timeout(Duration::from_millis(20)) {
            update(&mut app, msg);
        }
    }

    tui::restore_terminal()?;
    Ok(())
}

fn handle_event(app: &mut App) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(20))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key, app));
            }
        }
    }

    Ok(None)
}

fn handle_key(key: event::KeyEvent, app: &mut App) -> Option<Message> {
    match key.code {
        KeyCode::Tab => Some(Message::Switch),
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('y') => {
            app.break_duration += app.break_stop.unwrap() - app.break_start.unwrap();
            app.break_start = None;
            app.break_stop = None;
            Some(Message::Switch)
        }
        KeyCode::Char('n') => Some(Message::Switch),
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
        Message::Lock => app.break_start = Some(Local::now()),
        Message::Unlock => {
            if let Some(_) = app.break_start {
                app.break_stop = Some(Local::now());
                app.current_screen = State::Prompt;
                return Some(Message::Switch);
            }
        }
        Message::Quit => app.current_screen = State::Done,
    }
    None
}
