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
