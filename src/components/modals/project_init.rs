use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Wrap},
};
use super::SharedModal;

pub struct ProjectInitModal;

impl ProjectInitModal {
    pub fn render(f: &mut Frame, project_name: &str, cursor_visible: bool) {
        // Use shared modal styling
        let modal_area = SharedModal::create_modal_area(f, 60, 12, "Project Setup");
        let inner = SharedModal::create_inner_area(modal_area);

        let chunks = SharedModal::create_layout(
            inner,
            vec![
                Constraint::Length(2), // Welcome text
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Project label
                Constraint::Length(1), // Input field
                Constraint::Length(1), // Spacer
                Constraint::Length(2), // Instructions
            ],
        );

        // Welcome text
        let welcome_text = Text::from(vec![
            Line::from("Welcome to claudectl!"),
            Line::from("Initialize a new project to get started."),
        ]);
        let welcome = Paragraph::new(welcome_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(welcome, chunks[0]);

        // Project label
        let label = Paragraph::new("Project:")
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(label, chunks[2]);

        // Input field with cursor
        let mut input_spans = vec![Span::styled(project_name, Style::default().fg(Color::White))];

        if cursor_visible {
            input_spans.push(Span::styled("█", Style::default().fg(Color::White)));
        }

        let input_text = Text::from(Line::from(input_spans));
        let input_block = Block::default()
            .style(Style::default().bg(Color::Black));
        let input = Paragraph::new(input_text)
            .block(input_block)
            .wrap(Wrap { trim: false });
        f.render_widget(input, chunks[3]);

        // Instructions
        let instructions = Text::from(vec![
            Line::from("Press Enter to create project"),
            Line::from("Press Esc to quit"),
        ]);
        let instructions_widget = Paragraph::new(instructions)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray));
        f.render_widget(instructions_widget, chunks[5]);
    }
}
