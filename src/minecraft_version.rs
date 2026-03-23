use regex::Regex;

pub struct MinecraftVersion;

impl MinecraftVersion {
    pub fn validate(input: Option<String>) -> Result<String, &'static str> {
        match input {
            Some(input) => {
                let version_regex = Regex::new(r"^[1-9]?\d\.[1-9]?\d(?:\.[1-9]?\d)?$").unwrap();
                if version_regex.is_match(input.as_str()) {
                    Ok(input.to_string())
                } else {
                    Err("Invalid version")
                }
            }
            None => Err("No input"),
        }

    }
}

