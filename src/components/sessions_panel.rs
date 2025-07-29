use crate::data::{Session, SessionStatus};
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};

pub struct SessionsPanel;

impl SessionsPanel {
    pub fn render(
        f: &mut Frame, 
        area: ratatui::prelude::Rect, 
        sessions: &[Session],
        selected_index: Option<usize>,
        _is_focused: bool
    ) {
        let sessions_block = Block::default()
            .style(Style::default().bg(Color::Rgb(5, 5, 5))); // Darkest color, no border or title

        if sessions.is_empty() {
            let empty_content = Paragraph::new(Text::from(vec![
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("No active sessions", Style::default().fg(Color::Rgb(150, 150, 150)).italic()),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("[N] ", Style::default().fg(Color::Yellow).bold()),
                    Span::raw("New Session"),
                ]),
            ]))
            .block(sessions_block);
            
            f.render_widget(empty_content, area);
            return;
        }

        let items: Vec<ListItem> = sessions
            .iter()
            .enumerate()
            .map(|(i, session)| {
                let status_color = match session.status {
                    SessionStatus::Active => Color::Green,
                    SessionStatus::Stopped => Color::Yellow,
                    SessionStatus::Error => Color::Red,
                };

                let status_symbol = match session.status {
                    SessionStatus::Active => "●",
                    SessionStatus::Stopped => "⏸",
                    SessionStatus::Error => "✗",
                };

                let project_info = session.project_id
                    .as_ref().map_or_else(|| " (No project)".to_string(), |id| format!(" ({})", &id[..8.min(id.len())]));

                let is_selected = selected_index == Some(i);
                let style = if is_selected {
                    Style::default().bg(Color::Rgb(40, 40, 60))
                } else {
                    Style::default()
                };

                ListItem::new(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(status_symbol, Style::default().fg(status_color).bold()),
                    Span::raw(" "),
                    Span::styled(
                        format!("Session {}", &session.id[..8.min(session.id.len())]),
                        Style::default().fg(Color::White)
                    ),
                    Span::styled(project_info, Style::default().fg(Color::Rgb(150, 150, 150))),
                ])).style(style)
            })
            .collect();

        let sessions_list = List::new(items)
            .block(sessions_block)
            .highlight_style(Style::default().bg(Color::Rgb(50, 50, 80)));

        f.render_widget(sessions_list, area);
    }
}