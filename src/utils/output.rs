use crate::utils::icons::ICONS;
use owo_colors::OwoColorize;

pub fn blank() {
    println!();
}

pub fn error(message: &String) {
    eprintln!("{} {}", ICONS.status.failure.red().bold(), message.white());
}
