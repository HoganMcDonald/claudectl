use crate::data::{AppStats, SessionStatus, Session};
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub struct StatsPanel;

impl StatsPanel {
    pub fn render(
        f: &mut Frame, 
        area: ratatui::prelude::Rect, 
        stats: &AppStats,
        sessions: &[Session]
    ) {
        let stats_block = Block::default()
            .title("📊 Statistics")
            .title_style(Style::default().fg(Color::Rgb(255, 150, 100)).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(200, 120, 80)))
            .style(Style::default().bg(Color::Rgb(25, 20, 15)));

        // Calculate additional stats from sessions
        let active_sessions = sessions.iter().filter(|s| matches!(s.status, SessionStatus::Active)).count();
        let stopped_sessions = sessions.iter().filter(|s| matches!(s.status, SessionStatus::Stopped)).count();
        let error_sessions = sessions.iter().filter(|s| matches!(s.status, SessionStatus::Error)).count();

        let stats_text = Text::from(vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("📁 ", Style::default().fg(Color::Blue)),
                Span::styled("Projects: ", Style::default().fg(Color::Rgb(150, 150, 150))),
                Span::styled(
                    stats.total_projects.to_string(), 
                    Style::default().fg(Color::White).bold()
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("🔄 ", Style::default().fg(Color::Green)),
                Span::styled("Sessions:", Style::default().fg(Color::Rgb(150, 150, 150))),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("● ", Style::default().fg(Color::Green)),
                Span::styled("Active: ", Style::default().fg(Color::Rgb(150, 150, 150))),
                Span::styled(
                    active_sessions.to_string(), 
                    Style::default().fg(Color::Green).bold()
                ),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("⏸ ", Style::default().fg(Color::Yellow)),
                Span::styled("Stopped: ", Style::default().fg(Color::Rgb(150, 150, 150))),
                Span::styled(
                    stopped_sessions.to_string(), 
                    Style::default().fg(Color::Yellow).bold()
                ),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("✗ ", Style::default().fg(Color::Red)),
                Span::styled("Errors: ", Style::default().fg(Color::Rgb(150, 150, 150))),
                Span::styled(
                    error_sessions.to_string(), 
                    Style::default().fg(Color::Red).bold()
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("⏱️  ", Style::default().fg(Color::Cyan)),
                Span::styled("Runtime: ", Style::default().fg(Color::Rgb(150, 150, 150))),
                Span::styled(
                    Self::format_duration(stats.get_total_runtime()),
                    Style::default().fg(Color::Cyan).bold()
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("💾 ", Style::default().fg(Color::Magenta)),
                Span::styled("Memory: ", Style::default().fg(Color::Rgb(150, 150, 150))),
                Span::styled("~2MB", Style::default().fg(Color::Magenta).bold()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("🌐 ", Style::default().fg(Color::Blue)),
                Span::styled("Status: ", Style::default().fg(Color::Rgb(150, 150, 150))),
                Span::styled("Online", Style::default().fg(Color::Green).bold()),
            ]),
        ]);

        let stats_content = Paragraph::new(stats_text)
            .block(stats_block);

        f.render_widget(stats_content, area);
    }

    fn format_duration(duration: std::time::Duration) -> String {
        let total_seconds = duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if hours > 0 {
            format!("{hours}h {minutes}m")
        } else if minutes > 0 {
            format!("{minutes}m {seconds}s")
        } else {
            format!("{seconds}s")
        }
    }
}