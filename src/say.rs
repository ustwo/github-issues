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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_info() {
        assert_eq!(info(), "\u{1b}[32mInfo:\u{1b}[0m");
    }
    #[test]
    fn format_info() {
        assert_eq!(format!("{} {}", info(), "foo"), "\u{1b}[32mInfo:\u{1b}[0m foo");
    }
}
