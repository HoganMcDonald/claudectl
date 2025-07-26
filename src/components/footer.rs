use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

pub struct Footer;

impl Footer {
    pub fn render(f: &mut Frame, area: ratatui::prelude::Rect) {
        // Create horizontal split for statusline
        let statusline_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),        // Left side - commands
                Constraint::Length(20),    // Right side - status info
            ])
            .split(area);

        // Left side - key commands
        let commands_content = Paragraph::new(Line::from(vec![
            Span::styled("q", Style::default().fg(Color::Red).bold()),
            Span::raw("/"),
            Span::styled("ESC", Style::default().fg(Color::Red).bold()),
            Span::raw(" quit  "),
            Span::styled("?", Style::default().fg(Color::Magenta).bold()),
            Span::raw(" help  "),
            Span::styled("p", Style::default().fg(Color::Yellow).bold()),
            Span::raw(" add project  "),
            Span::styled("n", Style::default().fg(Color::Green).bold()),
            Span::raw(" new session"),
        ]))
        .style(Style::default().bg(Color::Rgb(20, 20, 30)).fg(Color::Rgb(200, 200, 200)))
        .alignment(Alignment::Left);

        // Right side - status info
        let status_content = Paragraph::new(Line::from(vec![
            Span::styled("Ready", Style::default().fg(Color::Green).bold()),
        ]))
        .style(Style::default().bg(Color::Rgb(20, 20, 30)).fg(Color::Rgb(200, 200, 200)))
        .alignment(Alignment::Right);

        f.render_widget(commands_content, statusline_chunks[0]);
        f.render_widget(status_content, statusline_chunks[1]);
    }
}
