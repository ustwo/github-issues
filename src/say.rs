use ansi_term::Colour::{Green, Yellow, Red, White};

pub fn info() -> String {
    Green.paint("Info:").to_string()
}

pub fn warn() -> String {
    Yellow.paint("Warning:").to_string()
}

pub fn error() -> String {
    Red.paint("Error:").to_string()
}

pub fn highlight(text: &str) -> String {
    White.bold().paint(text).to_string()
}
