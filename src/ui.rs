use chrono::{Local, TimeDelta};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, State};

pub fn ui(frame: &mut Frame, app: &App) {
    // let time_format = "%H:%M:%S // %Y-%m-%d";
    let time_format = "%H:%M:%S";
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        ":::Rustimer:::",
        Style::default().fg(Color::Rgb(234, 157, 52)),
    ))
    .alignment(Alignment::Center)
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    match app.current_screen {
        State::Main => {
            //main part ui
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Length(2),
                    Constraint::Length(2),
                    Constraint::Length(2),
                    Constraint::Min(1),
                ])
                .split(chunks[1]);

            let temp = Local::now() - app.start_time;
            let time_elapsed = Paragraph::new(Text::styled(
                format!(
                    "time_elapsed: {}:{}:{}",
                    temp.num_hours(),
                    temp.num_minutes(),
                    temp.num_seconds()
                ),
                Style::default().fg(Color::Rgb(49, 116, 143)),
            ))
            .alignment(Alignment::Center)
            .block(Block::default());

            let break_time = Paragraph::new(Text::styled(
                format!(
                    "break_time: {}:{}:{}",
                    app.break_duration.num_hours(),
                    app.break_duration.num_minutes(),
                    app.break_duration.num_seconds()
                ),
                Style::default().fg(Color::Rgb(49, 116, 143)),
            ))
            .alignment(Alignment::Center)
            .block(Block::default());

            let temp = (app.start_time + TimeDelta::hours(8)) - app.break_duration;
            let rem_time = Paragraph::new(Text::styled(
                format!("end_time: {}", temp.format(time_format)),
                Style::default().fg(Color::Rgb(49, 116, 143)),
            ))
            .alignment(Alignment::Center)
            .block(Block::default());

            let footer_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());

            let footer = Paragraph::new(Text::styled(
                "Quit (q)",
                Style::default().fg(Color::Rgb(235, 111, 146)),
            ))
            .alignment(Alignment::Center)
            .block(footer_block);
            frame.render_widget(time_elapsed, main_chunks[1]);
            frame.render_widget(break_time, main_chunks[2]);
            frame.render_widget(rem_time, main_chunks[3]);
            frame.render_widget(footer, chunks[2]);
        }
        State::Prompt => {
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Length(2),
                    Constraint::Min(1),
                ])
                .split(chunks[1]);

            let break_prompt = Paragraph::new(Text::styled(
                "Does this count??",
                Style::default().fg(Color::Rgb(49, 116, 143)),
            ))
            .alignment(Alignment::Center)
            .block(Block::default());

            let info = vec![
                Span::styled("Yes (y)", Style::default().fg(Color::Rgb(235, 111, 146))),
                Span::styled(" | ", Style::default().fg(Color::White)),
                Span::styled("No (n)", Style::default().fg(Color::Rgb(235, 111, 146))),
                Span::styled(" | ", Style::default().fg(Color::White)),
                Span::styled("Quit (q)", Style::default().fg(Color::Rgb(235, 111, 146))),
            ];
            let footer_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());

            let footer = Paragraph::new(Line::from(info))
                .alignment(Alignment::Center)
                .block(footer_block);

            frame.render_widget(break_prompt, main_chunks[1]);
            frame.render_widget(footer, chunks[2]);
        }
        State::Done => {}
    }
}
