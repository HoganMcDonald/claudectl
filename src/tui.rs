use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
};

use crate::app::{App, AppMode};
use crate::components::{
    Footer, Header, SessionsPanel, ProjectsPanel, StatsPanel,
    HelpModal, FilePickerModal, ConfirmationModal
};
use std::{error::Error, io};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if let Err(err) = app.handle_key_event(key.code) {
                // Handle app errors - for now just break on error
                eprintln!("App error: {}", err);
                break;
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    // Main layout with modern spacing
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(5), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(f.size());

    // Render components
    Header::render(f, main_chunks[0]);
    render_main_content(f, main_chunks[1], app);
    Footer::render(f, main_chunks[2]);

    // Render modals on top if active
    match app.mode {
        AppMode::HelpModal => {
            HelpModal::render(f);
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

fn render_main_content(f: &mut Frame, area: ratatui::prelude::Rect, app: &App) {
    // New 3-panel layout: Sessions (30%) | Projects (45%) | Stats (25%)
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Sessions
            Constraint::Percentage(45), // Projects  
            Constraint::Percentage(25), // Stats
        ])
        .split(area);

    // Render the three main panels
    SessionsPanel::render(
        f, 
        content_chunks[0], 
        &app.data.sessions,
        app.selected_session_index
    );

    ProjectsPanel::render(
        f, 
        content_chunks[1], 
        &app.data.projects,
        app.selected_project_index
    );

    StatsPanel::render(
        f, 
        content_chunks[2], 
        &app.data.stats,
        &app.data.sessions
    );
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App::new().map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let res = run_app(&mut terminal, app);

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
