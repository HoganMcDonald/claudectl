pub mod footer;
pub mod header;
pub mod quick_actions;
pub mod system_status;
pub mod workflow_panel;

// New components
pub mod sessions_panel;
pub mod projects_panel;
pub mod stats_panel;
pub mod modals;

pub use footer::Footer;
pub use header::Header;

// New component exports
pub use sessions_panel::SessionsPanel;
pub use modals::{HelpModal, FilePickerModal, ConfirmationModal};
