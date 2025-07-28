use crate::data::{Session, SessionStatus};
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

pub struct SessionsPanel;

impl SessionsPanel {
    pub fn render(
        f: &mut Frame, 
        area: ratatui::prelude::Rect, 
        sessions: &[Session],
        selected_index: Option<usize>,
        is_focused: bool
    ) {
        let title = if is_focused {
            "🔄 Sessions [FOCUSED]"
        } else {
            "🔄 Sessions"
        };

        let title_color = if is_focused {
            Color::Rgb(150, 255, 150)
        } else {
            Color::Rgb(100, 200, 100)
        };

        let border_color = if is_focused {
            Color::Rgb(120, 220, 120)
        } else {
            Color::Rgb(80, 160, 80)
        };

        let sessions_block = Block::default()
            .title(title)
            .title_style(Style::default().fg(title_color).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(Color::Rgb(15, 25, 15)));

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