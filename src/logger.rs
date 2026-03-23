use chrono::Local;
use log::LevelFilter;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Formats a log message with timestamp and log level
pub fn format_log_message(timestamp: &str, level: &str, message: &str) -> String {
    format!("[{}][{}] {}", timestamp, level, message)
}

/// Writes the log file header with initialization timestamp
pub fn write_log_header(log_file: &Path) -> std::io::Result<()> {
    let mut file = File::create(log_file)?;
    writeln!(
        file,
        "=== SSM Splitscreen Minecraft - {} ===",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    )?;
    Ok(())
}

pub fn init_logger(log_file: &Path) {
    // Create the log file and write header
    write_log_header(log_file).expect("Failed to write log header");

    // Configure the logger
    fern::Dispatch::new()
        .format(|out, message, record| {
            let formatted = format_log_message(
                &Local::now().format("%H:%M:%S").to_string(),
                &record.level().to_string(),
                &message.to_string(),
            );
            out.finish(format_args!("{}", formatted))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(File::create(log_file).expect("Failed to open log file"))
        .apply()
        .expect("Failed to set up log file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn format_log_message_formats_correctly() {
        let result = format_log_message("12:34:56", "INFO", "Test message");
        assert_eq!(result, "[12:34:56][INFO] Test message");
    }

    #[test]
    fn format_log_message_handles_different_levels() {
        let levels = vec!["DEBUG", "INFO", "WARN", "ERROR"];
        for level in levels {
            let result = format_log_message("10:20:30", level, "Test");
            assert!(result.contains(level));
            assert!(result.contains("10:20:30"));
            assert!(result.contains("Test"));
        }
    }

    #[test]
    fn format_log_message_preserves_message_content() {
        let messages = vec![
            "Simple message",
            "Message with special chars: !@#$%",
            "Message with spaces and numbers 12345",
        ];
        for msg in messages {
            let result = format_log_message("12:00:00", "INFO", msg);
            assert!(result.contains(msg));
        }
    }

    #[test]
    fn write_log_header_creates_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let result = write_log_header(path);
        assert!(result.is_ok());
        assert!(path.exists());
    }

    #[test]
    fn write_log_header_contains_timestamp() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        write_log_header(path).unwrap();

        let contents = std::fs::read_to_string(path).unwrap();
        assert!(contents.contains("=== SSM Splitscreen Minecraft -"));
        assert!(contents.contains("==="));
    }

    #[test]
    fn write_log_header_contains_correct_date_format() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        write_log_header(path).unwrap();

        let contents = std::fs::read_to_string(path).unwrap();
        // Check for YYYY-MM-DD HH:MM:SS format
        assert!(
            regex::Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}")
                .unwrap()
                .is_match(&contents)
        );
    }

    #[test]
    fn write_log_header_overwrites_existing_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Write initial content
        std::fs::write(path, "old content").unwrap();
        assert_eq!(std::fs::read_to_string(path).unwrap(), "old content");

        // Write header
        write_log_header(path).unwrap();
        let contents = std::fs::read_to_string(path).unwrap();

        // Should contain header, not old content
        assert!(contents.contains("=== SSM Splitscreen Minecraft"));
        assert!(!contents.contains("old content"));
    }

    #[test]

    fn format_log_message_with_empty_message() {
        let result = format_log_message("12:34:56", "INFO", "");
        assert_eq!(result, "[12:34:56][INFO] ");
    }

    #[test]
    fn format_log_message_with_multiline_message() {
        let result = format_log_message("12:34:56", "INFO", "Line1\nLine2");
        assert!(result.contains("Line1\nLine2"));
    }
}
