use crate::utils::{icons::ICONS, theme::THEME};
use owo_colors::OwoColorize;
use tabled::{
    Table, Tabled,
    settings::{Border, Modify, Remove, object::Rows, style::Style},
};

pub fn blank() {
    println!();
}

pub fn standard(message: &str) {
    println!("{}", message.color(THEME.text));
}

pub fn success(message: &str) {
    println!(
        "{} {}",
        ICONS.status.success.color(THEME.success).bold(),
        message.color(THEME.text)
    );
}

pub fn error(message: &str) {
    eprintln!(
        "{} {}",
        ICONS.status.failure.color(THEME.error).bold(),
        message.color(THEME.error)
    );
}

pub fn table<T: Tabled>(data: &[T], show_header: bool) {
    let mut table = Table::new(data);
    table.with(Style::empty());

    if show_header {
        table.with(Modify::new(Rows::first()).with(Border::new().bottom('─')));
    } else {
        table.with(Remove::row(Rows::first()));
    }

    println!("{table}");
}

pub enum Position {
    First,
    #[allow(dead_code)]
    Normal,
    #[allow(dead_code)]
    Last,
}

pub fn step(message: &str, position: Position) {
    let icon = match position {
        Position::First => ICONS.box_draw.corner_tl,
        Position::Normal => ICONS.box_draw.tee_left,
        Position::Last => ICONS.box_draw.corner_bl,
    };
    print!(
        "{} {}",
        icon.color(THEME.primary).bold(),
        message.color(THEME.muted)
    );
}

pub fn step_end() {
    print!("{} ", ICONS.status.success.color(THEME.success).bold());
}

pub fn step_skip() {
    print!("{} ", ICONS.arrows.right.color(THEME.info).bold());
}

pub fn step_fail() {
    print!("{} ", ICONS.status.failure.color(THEME.error).bold());
}
