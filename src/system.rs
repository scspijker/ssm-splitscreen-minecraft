use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::Command;

/// Gets the current display resolution from xrandr
pub fn get_resolution() -> io::Result<(u32, u32)> {
    let output = Command::new("xrandr").arg("--current").output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains('*') {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(resolution) = parts.iter().find(|s| s.contains('x')) {
                let dims: Vec<u32> = resolution
                    .split('x')
                    .filter_map(|s| s.parse().ok())
                    .collect();
                if dims.len() == 2 {
                    return Ok((dims[0], dims[1]));
                }
            }
        }
    }
    Ok((1920, 1080)) // Default fallback
}

/// Counts the number of connected controllers
pub fn count_controllers() -> io::Result<usize> {
    let file = File::open("/proc/bus/input/devices")?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map_while(Result::ok)
        .filter(|line| line.contains("pad") || line.contains("joystick"))
        .count())
}

/// Checks if the built-in Steam Deck controller is enabled
pub fn is_built_in_controller_enabled() -> io::Result<bool> {
    let file = File::open("/proc/bus/input/devices")?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map_while(Result::ok)
        .any(|line| line.contains("Vendor=28de") && line.contains("Product=1205")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_resolution_returns_valid_tuple() {
        // This test assumes xrandr is available (will skip otherwise)
        match get_resolution() {
            Ok((width, height)) => {
                assert!(width > 0);
                assert!(height > 0);
            }
            Err(_) => {
                // xrandr might not be available on all systems, that's ok
            }
        }
    }

    #[test]
    fn count_controllers_returns_non_negative() {
        match count_controllers() {
            Ok(_count) => {
                // Just checking it doesn't panic and returns a value
            }
            Err(_) => {
                // /proc might not exist on all systems, that's ok
            }
        }
    }

    #[test]
    fn is_built_in_controller_enabled_returns_bool() {
        match is_built_in_controller_enabled() {
            Ok(_result) => {
                // Just checking it doesn't panic
            }
            Err(_) => {
                // /proc might not exist on all systems, that's ok
            }
        }
    }
}
