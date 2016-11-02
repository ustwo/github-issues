use std::str::FromStr;

pub enum OutputFormat {
    CSV,
    JSON,
}

impl FromStr for OutputFormat {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "csv" => Ok(OutputFormat::CSV),
            "json" => Ok(OutputFormat::JSON),
            _     => Err("Unexpected output format")
        }
    }
}


// CSV lib does not cast Vec<String> so this is a workaround.
pub fn split(s: String) -> Vec<String> {
    let s = s.trim();

    if s.is_empty() {
        vec![]
    } else {
        s.split(",").map(|s| From::from(s.trim())).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let expected: Vec<String> = vec![];
        assert_eq!(split("".to_string()), expected);
    }

    #[test]
    fn string_with_spaces() {
        let expected: Vec<String> = vec![];
        assert_eq!(split("  ".to_string()), expected);
    }

    #[test]
    fn string_with_one_item() {
        let expected: Vec<String> = vec![String::from("one")];
        assert_eq!(split("one".to_string()), expected);
    }

    #[test]
    fn string_with_multiple_item() {
        let expected: Vec<String> = vec![ String::from("one")
                                        , String::from("two")
                                        , String::from("three") ];
        assert_eq!(split("one,two, three".to_string()), expected);
    }

}
