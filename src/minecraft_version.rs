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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1.21", true)]
    #[case("1.21.1", true)]
    #[case("1.20", true)]
    #[case("1.20.4", true)]
    #[case("10.20", true)]
    #[case("10.20.30", true)]
    #[case("99.99", true)]
    #[case("99.99.99", true)]
    #[case("1.12", true)]
    #[case("1.12.2", true)]
    #[case("0.1", true)]
    #[case("1.0", true)]
    #[case("1.0.0", true)]
    fn validate_accepts_valid_versions(#[case] version: &str, #[case] should_pass: bool) {
        let result = MinecraftVersion::validate(Some(version.to_string()));
        assert_eq!(result.is_ok(), should_pass);
        if should_pass {
            assert_eq!(result.unwrap(), version);
        }
    }

    #[rstest]
    #[case("1")]
    #[case("1.2.3.4")]
    #[case("a.b.c")]
    #[case("1.a.2")]
    #[case("v1.21")]
    #[case("1.21.")]
    #[case(".1.21")]
    #[case("1..21")]
    #[case("")]
    #[case("-1.21")]
    #[case("1.21 ")]
    #[case(" 1.21")]
    fn validate_rejects_invalid_versions(#[case] version: &str) {
        let result = MinecraftVersion::validate(Some(version.to_string()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid version");
    }

    #[test]
    fn validate_returns_no_input_error_for_none() {
        let result = MinecraftVersion::validate(None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No input");
    }

    #[test]
    fn validate_returns_same_string_on_success() {
        let test_version = "1.21.1".to_string();
        let result = MinecraftVersion::validate(Some(test_version.clone()));
        assert_eq!(result.unwrap(), test_version);
    }
}
