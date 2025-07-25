use crate::app::FilePickerState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub struct FilePickerModal;

impl FilePickerModal {
    pub fn render(f: &mut Frame, picker_state: &FilePickerState) {
        let area = f.size();
        
        // Create centered modal area (80% width, 70% height)
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
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(modal_area)[1];

        // Clear the area behind the modal
        f.render_widget(Clear, modal_area);

        // Split modal into header, content, and footer
        let modal_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(0),     // Content
                Constraint::Length(3),  // Footer
            ])
            .split(modal_area);

        // Render header with current path
        let header_block = Block::default()
            .title("📁 Select Project Directory")
            .title_style(Style::default().fg(Color::Yellow).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow))
            .style(Style::default().bg(Color::Rgb(30, 25, 20)));

        let current_path = picker_state.current_path.to_string_lossy();
        let path_display = if current_path.len() > 60 {
            format!("...{}", &current_path[current_path.len() - 57..])
        } else {
            current_path.to_string()
        };

        let header_content = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Current: ", Style::default().fg(Color::Rgb(150, 150, 150))),
                Span::styled(path_display, Style::default().fg(Color::White).bold()),
            ]),
        ])
        .block(header_block)
        .alignment(Alignment::Left);

        f.render_widget(header_content, modal_chunks[0]);

        // Render directory entries
        let entries_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Blue))
            .style(Style::default().bg(Color::Rgb(20, 20, 30)));

        if picker_state.entries.is_empty() {
            let empty_content = Paragraph::new("Directory is empty or unreadable")
                .block(entries_block)
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Rgb(150, 150, 150)));
            f.render_widget(empty_content, modal_chunks[1]);
        } else {
            let items: Vec<ListItem> = picker_state
                .entries
                .iter()
                .enumerate()
                .map(|(i, entry)| {
                    let is_selected = i == picker_state.selected_index;
                    let style = if is_selected {
                        Style::default().bg(Color::Rgb(40, 40, 60))
                    } else {
                        Style::default()
                    };

                    let (icon, icon_color) = if entry.is_directory {
                        if entry.name == ".." {
                            ("📁", Color::Yellow)
                        } else {
                            ("📁", Color::Blue)
                        }
                    } else {
                        ("📄", Color::White)
                    };

                    let name_style = if entry.is_directory {
                        Style::default().fg(Color::Cyan).bold()
                    } else {
                        Style::default().fg(Color::Rgb(200, 200, 200))
                    };

                    ListItem::new(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(icon, Style::default().fg(icon_color)),
                        Span::raw(" "),
                        Span::styled(&entry.name, name_style),
                    ])).style(style)
                })
                .collect();

            let entries_list = List::new(items)
                .block(entries_block)
                .highlight_style(Style::default().bg(Color::Rgb(50, 50, 80)));

            f.render_widget(entries_list, modal_chunks[1]);
        }

        // Render footer with controls
        let footer_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(100, 100, 100)))
            .style(Style::default().bg(Color::Rgb(25, 25, 35)));

        let footer_content = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Enter:", Style::default().fg(Color::Green).bold()),
                Span::raw(" Select/Navigate  "),
                Span::styled("Backspace:", Style::default().fg(Color::Yellow).bold()),
                Span::raw(" Go Up  "),
                Span::styled("ESC:", Style::default().fg(Color::Red).bold()),
                Span::raw(" Cancel"),
            ]),
        ])
        .block(footer_block)
        .alignment(Alignment::Center);

        f.render_widget(footer_content, modal_chunks[2]);

        // Show error message if any
        if let Some(ref error) = picker_state.error_message {
            let error_area = ratatui::prelude::Rect {
                x: modal_area.x + 2,
                y: modal_area.y + modal_area.height - 5,
                width: modal_area.width - 4,
                height: 3,
            };

            let error_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .style(Style::default().bg(Color::Rgb(40, 20, 20)));

            let error_content = Paragraph::new(error.clone())
                .block(error_block)
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center);

            f.render_widget(error_content, error_area);
        }
    }
}