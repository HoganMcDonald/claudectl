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
    let red = Rgb(255, 50, 50);  // Vibrant red
    eprintln!("{} {}", ICONS.status.failure.color(red).bold(), message.color(red));
}

pub fn info(message: &str) {
    let blue = Rgb(70, 130, 255);  // Vibrant blue
    let muted = Rgb(150, 150, 150);
    eprintln!("{} {}", ICONS.status.info.color(blue).bold() , message.color(muted));
}
