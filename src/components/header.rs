use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct Header;

impl Header {
    pub fn render(f: &mut Frame, area: ratatui::prelude::Rect) {
        let header_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(100, 200, 255)))
            .style(Style::default().bg(Color::Rgb(20, 25, 35)));

        let header_content = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("⚡ ", Style::default().fg(Color::Yellow).bold()),
                Span::styled(
                    "claudectl",
                    Style::default()
                        .fg(Color::Rgb(100, 200, 255))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" v0.1.0", Style::default().fg(Color::Rgb(120, 120, 120))),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    "Multi-Agent Workflow Orchestrator",
                    Style::default().fg(Color::Rgb(180, 180, 180)).italic(),
                ),
            ]),
        ])
        .block(header_block)
        .alignment(Alignment::Left);

        f.render_widget(header_content, area);
    }
}
