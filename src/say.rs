use ansi_term::Colour::{Green, Yellow, Red, White};
use ansi_term::ANSIString;

pub fn info() -> String {
    Green.paint("Info:").to_string()
}

pub fn warn() -> String {
    Yellow.paint("Warning:").to_string()
}

pub fn error() -> String {
    Red.paint("Error:").to_string()
}
