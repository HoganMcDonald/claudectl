use ratatui::{
    Frame,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct QuickActions;

impl QuickActions {
    pub fn render(f: &mut Frame, area: ratatui::prelude::Rect) {
        let actions_block = Block::default()
            .title("⚡ Quick Actions")
            .title_style(Style::default().fg(Color::Yellow).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow))
            .style(Style::default().bg(Color::Rgb(25, 20, 15)));

        let actions_text = Text::from(vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[N] ", Style::default().fg(Color::Yellow).bold()),
                Span::raw("New Workflow"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[L] ", Style::default().fg(Color::Blue).bold()),
                Span::raw("Load Template"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[S] ", Style::default().fg(Color::Green).bold()),
                Span::raw("Settings"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[?] ", Style::default().fg(Color::Magenta).bold()),
                Span::raw("Help"),
            ]),
        ]);

        let actions_content = Paragraph::new(actions_text).block(actions_block);

        f.render_widget(actions_content, area);
    }
}
