use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

pub struct ProjectInitModal;

impl ProjectInitModal {
    pub fn render(f: &mut Frame, project_name: &str, cursor_visible: bool) {
        let area = f.size();

        // Create centered modal area
        let modal_width = 60;
        let modal_height = 12;
        let x = (area.width.saturating_sub(modal_width)) / 2;
        let y = (area.height.saturating_sub(modal_height)) / 2;

        let modal_area = ratatui::layout::Rect {
            x,
            y,
            width: modal_width,
            height: modal_height,
        };

        // Clear the background
        f.render_widget(Clear, modal_area);

        // Main modal block
        let block = Block::default()
            .title(" Project Setup ")
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(Color::DarkGray).fg(Color::White));

        f.render_widget(block, modal_area);

        // Inner content area
        let inner = modal_area.inner(&Margin { vertical: 1, horizontal: 2 });

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2), // Welcome text
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Project label
                Constraint::Length(1), // Input field
                Constraint::Length(1), // Spacer
                Constraint::Length(2), // Instructions
            ])
            .split(inner);

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
