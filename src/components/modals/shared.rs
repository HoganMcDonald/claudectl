use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Style},
    widgets::{Block, Clear},
};

pub struct SharedModal;

impl SharedModal {
    /// Creates a centered modal area with consistent styling
    pub fn create_modal_area(
        f: &mut Frame,
        width: u16,
        height: u16,
        title: &str,
    ) -> ratatui::layout::Rect {
        let area = f.size();
        
        // Calculate centered position
        let x = (area.width.saturating_sub(width)) / 2;
        let y = (area.height.saturating_sub(height)) / 2;

        let modal_area = ratatui::layout::Rect {
            x,
            y,
            width,
            height,
        };

        // Clear the background
        f.render_widget(Clear, modal_area);

        // Main modal block with consistent styling
        let block = Block::default()
            .title(format!(" {} ", title))
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(Color::DarkGray).fg(Color::White));

        f.render_widget(block, modal_area);

        modal_area
    }

    /// Creates the inner content area with consistent margins
    pub fn create_inner_area(modal_area: ratatui::layout::Rect) -> ratatui::layout::Rect {
        modal_area.inner(&Margin { vertical: 1, horizontal: 2 })
    }

    /// Creates a standard vertical layout for modal content
    pub fn create_layout(
        inner_area: ratatui::layout::Rect,
        constraints: Vec<Constraint>,
    ) -> std::rc::Rc<[ratatui::layout::Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_area)
    }
}