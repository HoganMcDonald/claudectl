use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct Footer;

impl Footer {
    pub fn render(f: &mut Frame, area: ratatui::prelude::Rect) {
        let footer_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(80, 80, 80)))
            .style(Style::default().bg(Color::Rgb(25, 25, 35)));

        let footer_content = Paragraph::new(vec![Line::from(vec![
            Span::raw("  "),
            Span::styled("Press ", Style::default().fg(Color::Rgb(150, 150, 150))),
            Span::styled("q", Style::default().fg(Color::Red).bold()),
            Span::styled(" or ", Style::default().fg(Color::Rgb(150, 150, 150))),
            Span::styled("ESC", Style::default().fg(Color::Red).bold()),
            Span::styled(" to quit", Style::default().fg(Color::Rgb(150, 150, 150))),
            Span::raw("  •  "),
            Span::styled("?", Style::default().fg(Color::Magenta).bold()),
            Span::styled(" for help", Style::default().fg(Color::Rgb(150, 150, 150))),
        ])])
        .block(footer_block)
        .alignment(Alignment::Left);

        f.render_widget(footer_content, area);
    }
}
