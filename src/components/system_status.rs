use ratatui::{
    Frame,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct SystemStatus;

impl SystemStatus {
    pub fn render(f: &mut Frame, area: ratatui::prelude::Rect) {
        let status_block = Block::default()
            .title("📊 System")
            .title_style(Style::default().fg(Color::Green).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Green))
            .style(Style::default().bg(Color::Rgb(15, 25, 15)));

        let status_text = Text::from(vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("● ", Style::default().fg(Color::Green)),
                Span::raw("System Online"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("● ", Style::default().fg(Color::Green)),
                Span::raw("Agent Pool Ready"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("● ", Style::default().fg(Color::Yellow)),
                Span::raw("No Active Workflows"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Memory: "),
                Span::styled("12MB", Style::default().fg(Color::Cyan)),
            ]),
        ]);

        let status_content = Paragraph::new(status_text).block(status_block);

        f.render_widget(status_content, area);
    }
}
