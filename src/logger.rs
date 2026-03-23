use log::{LevelFilter, SetLoggerError};
use log::Level;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use chrono::Local;

pub fn init_logger(log_file: &Path) {
    // Create or truncate the log file
    let mut file = File::create(log_file).expect("Failed to create log file");
    writeln!(file, "=== SSM Splitscreen Minecraft - {} ===", Local::now().format("%Y-%m-%d %H:%M:%S"))
        .expect("Failed to write to log file");

    // Configure the logger
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                Local::now().format("%H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(File::create(log_file).expect("Failed to open log file"))
        .apply()
        .expect("Failed to set up log file");
}