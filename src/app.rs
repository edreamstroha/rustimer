use chrono::{DateTime, Duration, Local};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Main,
    Prompt,
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Switch,
    Lock,
    Unlock,
    Quit,
}

#[derive(Debug, Clone)]
pub struct App {
    pub current_screen: State,
    pub start_time: DateTime<Local>,
    pub break_duration: Duration,
    pub break_start: Option<DateTime<Local>>,
    pub break_stop: Option<DateTime<Local>>,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: State::Main,
            start_time: Local::now(),
            break_duration: Duration::seconds(0),
            break_start: None,
            break_stop: None,
        }
    }
}
