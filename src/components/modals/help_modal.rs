use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

pub struct HelpModal;

impl HelpModal {
    pub fn render(f: &mut Frame) {
        let area = f.size();
        
        // Create centered modal area (60% width, 70% height)
        let modal_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ])
            .split(area)[1];

        let modal_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(modal_area)[1];

        // Clear the area behind the modal
        f.render_widget(Clear, modal_area);

        let help_block = Block::default()
            .title("❓ Keyboard Shortcuts")
            .title_style(Style::default().fg(Color::Cyan).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan))
            .style(Style::default().bg(Color::Rgb(20, 20, 40)));

        let help_content = Text::from(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  General:", Style::default().fg(Color::Yellow).bold()),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("q, ESC", Style::default().fg(Color::Green).bold()),
                Span::raw("      Quit application"),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("?", Style::default().fg(Color::Green).bold()),
                Span::raw("            Toggle this help menu"),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("m", Style::default().fg(Color::Green).bold()),
                Span::raw("            Toggle metrics modal"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Projects:", Style::default().fg(Color::Blue).bold()),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("p", Style::default().fg(Color::Green).bold()),
                Span::raw("            Add new project"),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("d", Style::default().fg(Color::Green).bold()),
                Span::raw("            Remove selected project"),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("↑/↓", Style::default().fg(Color::Green).bold()),
                Span::raw("          Navigate project list"),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("Enter", Style::default().fg(Color::Green).bold()),
                Span::raw("        Select/Open project"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Sessions:", Style::default().fg(Color::Magenta).bold()),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("n", Style::default().fg(Color::Green).bold()),
                Span::raw("            New session"),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("s", Style::default().fg(Color::Green).bold()),
                Span::raw("            Stop selected session"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  File Picker:", Style::default().fg(Color::Cyan).bold()),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("↑/↓", Style::default().fg(Color::Green).bold()),
                Span::raw("          Navigate entries"),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("Enter", Style::default().fg(Color::Green).bold()),
                Span::raw("        Enter directory/Select"),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("Backspace", Style::default().fg(Color::Green).bold()),
                Span::raw("    Go up one level"),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("ESC", Style::default().fg(Color::Green).bold()),
                Span::raw("          Cancel selection"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Press ", Style::default().fg(Color::Rgb(150, 150, 150))),
                Span::styled("?", Style::default().fg(Color::Cyan).bold()),
                Span::styled(" or ", Style::default().fg(Color::Rgb(150, 150, 150))),
                Span::styled("ESC", Style::default().fg(Color::Red).bold()),
                Span::styled(" to close", Style::default().fg(Color::Rgb(150, 150, 150))),
            ]),
        ]);

        let help_paragraph = Paragraph::new(help_content)
            .block(help_block)
            .alignment(Alignment::Left);

        f.render_widget(help_paragraph, modal_area);
    }
}