use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
};
use tokio::time::{Duration, Instant};

use crate::app::{App, AppMode};
use crate::components::{
    Footer, Header, SessionsPanel,
    HelpModal, FilePickerModal, ConfirmationModal,
};
use crate::components::modals::MetricsModal;
use crate::components::modals::ProjectInitModal;
use std::{error::Error, io};

pub async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    // Restore active sessions on startup
    if let Err(e) = app.restore_active_sessions().await {
        eprintln!("Failed to restore active sessions: {e}");
    }

    // Set up periodic session status sync
    let mut last_sync = Instant::now();
    let sync_interval = Duration::from_secs(10);
    
    loop {
        terminal.draw(|f| ui(f, &app))?;

        // Use a timeout for event reading to allow periodic tasks
        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let Err(err) = app.handle_key_event(key.code) {
                    // Handle app errors - for now just break on error
                    eprintln!("App error: {err}");
                    break;
                }
            }
        }

        // Periodically sync session statuses
        if last_sync.elapsed() >= sync_interval {
            if let Err(e) = app.sync_session_statuses().await {
                eprintln!("Failed to sync session statuses: {e}");
            }
            last_sync = Instant::now();
        }

        if app.should_quit {
            // Cleanup processes before quitting
            app.cleanup_on_shutdown().await;
            break;
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    if app.mode == AppMode::ProjectInitModal {
        // Render project initialization modal with blank background
        f.render_widget(
            ratatui::widgets::Block::default().style(ratatui::style::Style::default().bg(ratatui::style::Color::Black)),
            f.size()
        );
        ProjectInitModal::render(f, &app.project_init_name, app.project_init_cursor_visible);
    } else {
        // Render normal layout
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Main content
                Constraint::Length(3), // Footer
            ])
            .split(f.size());

        // Render components
        let project_name = app.get_current_project_name();
        Header::render(f, main_chunks[0], &project_name);
        render_main_content(f, main_chunks[1], app);
        Footer::render(f, main_chunks[2]);

        // Render modals on top if active
        match app.mode {
            AppMode::HelpModal => {
                HelpModal::render(f);
            }
            AppMode::MetricsModal => {
                MetricsModal::render(f, f.size());
            }
            AppMode::FilePickerModal => {
                if let Some(ref picker_state) = app.file_picker_state {
                    FilePickerModal::render(f, picker_state);
                }
            }
            AppMode::ConfirmationModal(ref message) => {
                ConfirmationModal::render(f, message);
            }
            _ => {}
        }
    }
}

fn render_main_content(f: &mut Frame, area: ratatui::prelude::Rect, app: &App) {
    // New 2-panel layout: Sessions sidebar (25%) | Empty main area (75%)
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // Sessions sidebar
            Constraint::Percentage(75), // Empty main content area
        ])
        .split(area);

    // Render sessions sidebar
    SessionsPanel::render(
        f,
        content_chunks[0],
        &app.session_data.sessions,
        app.selected_session_index,
        matches!(app.focus_area, crate::app::FocusArea::Sessions)
    );

    // Render main content area
    render_main_content_area(f, content_chunks[1], app);
}

fn render_main_content_area(f: &mut Frame, area: ratatui::prelude::Rect, app: &App) {
    use ratatui::{
        text::{Line, Span, Text},
        widgets::{Block, BorderType, Borders, Paragraph, Wrap},
        style::{Color, Style, Stylize},
        layout::Alignment,
    };

    let title = if app.selected_session_output.is_some() {
        "📝 Session Output"
    } else {
        "📝 Main Content"
    };

    let content_block = Block::default()
        .title(title)
        .title_style(Style::default().fg(Color::Rgb(100, 150, 200)).bold())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 80)))
        .style(Style::default().bg(Color::Rgb(15, 15, 25)));

    let content = if let Some(ref output) = app.selected_session_output {
        // Show session output
        Paragraph::new(output.clone())
            .block(content_block)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White))
    } else {
        // Show empty state with instructions
        
        
        Paragraph::new(Text::from(vec![
            Line::from(""),
            Line::from(""),
            Line::from(vec![
                Span::styled("Session Output", Style::default().fg(Color::Rgb(150, 150, 150)).italic()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Navigate to a session using "),
                Span::styled("j/k", Style::default().fg(Color::Yellow).bold()),
                Span::raw(" or "),
                Span::styled("↑/↓", Style::default().fg(Color::Yellow).bold()),
            ]),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled("Enter", Style::default().fg(Color::Green).bold()),
                Span::raw(" to view session output"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Use "),
                Span::styled("Tab", Style::default().fg(Color::Cyan).bold()),
                Span::raw(" to switch focus between areas"),
            ]),
        ]))
        .block(content_block)
        .alignment(Alignment::Center)
    };

    f.render_widget(content, area);
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App::new().map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let res = run_app(&mut terminal, app).await;

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
