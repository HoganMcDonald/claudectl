use crate::utils::icons::ICONS;
use owo_colors::{OwoColorize, Rgb};

pub fn blank() {
    println!();
}

pub fn standard(message: &str) {
    let white = Rgb(240, 240, 240);
    println!("{}", message.color(white));
}

pub fn error(message: &str) {
    let red = Rgb(255, 50, 50); // Vibrant red
    eprintln!(
        "{} {}",
        ICONS.status.failure.color(red).bold(),
        message.color(red)
    );
}

pub enum Position {
    First,
    #[allow(dead_code)]
    Normal,
    #[allow(dead_code)]
    Last,
}

pub fn step(message: &str, last: Position) {
    let blue = Rgb(70, 130, 255); // Vibrant blue
    let muted = Rgb(150, 150, 150);
    let icon = match last {
        Position::First => ICONS.box_draw.corner_tl,
        Position::Normal => ICONS.box_draw.tee_left,
        Position::Last => ICONS.box_draw.corner_bl,
    };
    println!("{} {}", icon.color(blue).bold(), message.color(muted));
}
