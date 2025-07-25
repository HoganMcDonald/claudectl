use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

pub struct ConfirmationModal;

impl ConfirmationModal {
    pub fn render(f: &mut Frame, message: &str) {
        let area = f.size();
        
        // Create small centered modal area
        let modal_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Length(7),
                Constraint::Percentage(40),
            ])
            .split(area)[1];

        let modal_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(modal_area)[1];

        // Clear the area behind the modal
        f.render_widget(Clear, modal_area);

        let confirmation_block = Block::default()
            .title("⚠️  Confirmation")
            .title_style(Style::default().fg(Color::Yellow).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow))
            .style(Style::default().bg(Color::Rgb(40, 30, 20)));

        let confirmation_content = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(message, Style::default().fg(Color::White).bold()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("[Y]", Style::default().fg(Color::Green).bold()),
                Span::raw("es  "),
                Span::styled("[N]", Style::default().fg(Color::Red).bold()),
                Span::raw("o  "),
                Span::styled("[ESC]", Style::default().fg(Color::Red).bold()),
                Span::raw(" Cancel"),
            ]),
        ])
        .block(confirmation_block)
        .alignment(Alignment::Center);

        f.render_widget(confirmation_content, modal_area);
    }
}