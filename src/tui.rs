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

use crate::app::{App, AppMode};
use crate::components::{
    Footer, Header, SessionsPanel,
    HelpModal, FilePickerModal, ConfirmationModal,
};
use crate::components::modals::ProjectInitModal;
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
    match app.mode {
        AppMode::ProjectInitModal => {
            // Render project initialization modal with blank background
            f.render_widget(
                ratatui::widgets::Block::default().style(ratatui::style::Style::default().bg(ratatui::style::Color::Black)),
                f.size()
            );
            ProjectInitModal::render(f, &app.project_init_name, app.project_init_cursor_visible);
        }
        _ => {
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
        &app.data.sessions,
        app.selected_session_index
    );

    // Render empty main content area
    render_empty_content(f, content_chunks[1]);
}

fn render_empty_content(f: &mut Frame, area: ratatui::prelude::Rect) {
    use ratatui::{
        text::{Line, Span, Text},
        widgets::{Block, BorderType, Borders, Paragraph},
        style::{Color, Style, Stylize},
        layout::Alignment,
    };

    let empty_block = Block::default()
        .title("📝 Main Content")
        .title_style(Style::default().fg(Color::Rgb(100, 150, 200)).bold())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 80)))
        .style(Style::default().bg(Color::Rgb(15, 15, 25)));

    let empty_content = Paragraph::new(Text::from(vec![
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("Content area", Style::default().fg(Color::Rgb(150, 150, 150)).italic()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("This space is reserved for future content.", Style::default().fg(Color::Rgb(120, 120, 120))),
        ]),
    ]))
    .block(empty_block)
    .alignment(Alignment::Center);

    f.render_widget(empty_content, area);
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
