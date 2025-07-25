use ratatui::{
    Frame,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

pub struct WorkflowPanel;

impl WorkflowPanel {
    pub fn render(f: &mut Frame, area: ratatui::prelude::Rect) {
        let main_block = Block::default()
            .title("🔀 Workflows")
            .title_style(Style::default().fg(Color::Rgb(100, 200, 255)).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(80, 120, 160)))
            .style(Style::default().bg(Color::Rgb(15, 20, 30)));

        let welcome_text = Text::from(vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    "Welcome to claudectl!",
                    Style::default().fg(Color::White).bold(),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("🤖 ", Style::default().fg(Color::Green)),
                Span::raw("Multi-agent orchestration platform"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("⚙️  ", Style::default().fg(Color::Blue)),
                Span::raw("Workflow automation & management"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("🔗 ", Style::default().fg(Color::Yellow)),
                Span::raw("Agent coordination & communication"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("Status: ", Style::default().fg(Color::Rgb(120, 120, 120))),
                Span::styled("Ready", Style::default().fg(Color::Green).bold()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    "📝 Coming soon:",
                    Style::default().fg(Color::Rgb(255, 165, 0)).italic(),
                ),
            ]),
            Line::from(vec![Span::raw("    • Agent workflow designer")]),
            Line::from(vec![Span::raw("    • Real-time execution monitoring")]),
            Line::from(vec![Span::raw("    • Workflow templates & presets")]),
        ]);

        let main_content = Paragraph::new(welcome_text)
            .block(main_block)
            .wrap(Wrap { trim: true });

        f.render_widget(main_content, area);
    }
}
