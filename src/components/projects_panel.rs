use crate::data::Project;
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

pub struct ProjectsPanel;

impl ProjectsPanel {
    pub fn render(
        f: &mut Frame, 
        area: ratatui::prelude::Rect, 
        projects: &[Project],
        selected_index: Option<usize>
    ) {
        let projects_block = Block::default()
            .title("📁 Projects")
            .title_style(Style::default().fg(Color::Rgb(100, 150, 255)).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(80, 120, 200)))
            .style(Style::default().bg(Color::Rgb(15, 20, 30)));

        if projects.is_empty() {
            let empty_content = Paragraph::new(Text::from(vec![
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("No projects added", Style::default().fg(Color::Rgb(150, 150, 150)).italic()),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("[P] ", Style::default().fg(Color::Yellow).bold()),
                    Span::raw("Add Project"),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::raw("  Projects are repositories or"),
                ]),
                Line::from(vec![
                    Span::raw("  directories you want to track"),
                ]),
            ]))
            .block(projects_block);
            
            f.render_widget(empty_content, area);
            return;
        }

        let items: Vec<ListItem> = projects
            .iter()
            .enumerate()
            .map(|(i, project)| {
                let is_selected = selected_index == Some(i);
                let style = if is_selected {
                    Style::default().bg(Color::Rgb(40, 40, 60))
                } else {
                    Style::default()
                };

                // Check if project still exists
                let (status_symbol, status_color) = if project.exists() {
                    ("📁", Color::Green)
                } else {
                    ("⚠️", Color::Red)
                };

                // Truncate path for display
                let path_display = if project.path.to_string_lossy().len() > 35 {
                    format!("...{}", &project.path.to_string_lossy()[project.path.to_string_lossy().len() - 32..])
                } else {
                    project.path.to_string_lossy().to_string()
                };

                ListItem::new(vec![
                    Line::from(vec![
                        Span::raw("  "),
                        Span::raw(status_symbol),
                        Span::raw(" "),
                        Span::styled(&project.name, Style::default().fg(Color::White).bold()),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled(path_display, Style::default().fg(Color::Rgb(150, 150, 150))),
                    ]),
                ]).style(style)
            })
            .collect();

        let projects_list = List::new(items)
            .block(projects_block.clone())
            .highlight_style(Style::default().bg(Color::Rgb(50, 50, 80)));

        f.render_widget(projects_list, area);

        // Render controls at the bottom of the area
        let controls_area = ratatui::prelude::Rect {
            x: area.x + 1,
            y: area.y + area.height.saturating_sub(3),
            width: area.width.saturating_sub(2),
            height: 2,
        };

        let controls_text = Text::from(vec![
            Line::from(vec![
                Span::styled("[P] ", Style::default().fg(Color::Yellow).bold()),
                Span::raw("Add  "),
                Span::styled("[D] ", Style::default().fg(Color::Red).bold()),
                Span::raw("Remove  "),
                Span::styled("[↑↓] ", Style::default().fg(Color::Cyan).bold()),
                Span::raw("Navigate"),
            ]),
        ]);

        let controls = Paragraph::new(controls_text);
        f.render_widget(controls, controls_area);
    }
}