use log::error;

/// Calculates window positioning and sizing arguments based on player count and resolution
pub fn calculate_window_args(
    width: u32,
    half_width: u32,
    half_height: u32,
    players: usize,
) -> Vec<String> {
    match players {
        1 => vec!["--fullscreen".to_string()],
        2 => vec![
            format!("--width={} --height={} --x=0 --y=0", width, half_height),
            format!(
                "--width={} --height={} --x=0 --y={}",
                width, half_height, half_height
            ),
        ],
        3 => vec![
            format!(
                "--width={} --height={} --x=0 --y=0",
                half_width, half_height
            ),
            format!(
                "--width={} --height={} --x={} --y=0",
                half_width, half_height, half_width
            ),
            format!(
                "--width={} --height={} --x=0 --y={}",
                half_width, half_height, half_height
            ),
        ],
        4 => vec![
            format!(
                "--width={} --height={} --x=0 --y=0",
                half_width, half_height
            ),
            format!(
                "--width={} --height={} --x={} --y=0",
                half_width, half_height, half_width
            ),
            format!(
                "--width={} --height={} --x=0 --y={}",
                half_width, half_height, half_height
            ),
            format!(
                "--width={} --height={} --x={} --y={}",
                half_width, half_height, half_width, half_height
            ),
        ],
        _ => {
            error!(
                "Currently only 1-4 players are supported. Requested {}, exiting..",
                players
            );
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_window_args_fullscreen_for_one_player() {
        let args = calculate_window_args(1920, 960, 540, 1);
        assert_eq!(args.len(), 1);
        assert_eq!(args[0], "--fullscreen");
    }

    #[test]
    fn calculate_window_args_two_player_vertical_split() {
        let args = calculate_window_args(1920, 960, 540, 2);
        assert_eq!(args.len(), 2);
        assert!(args[0].contains("--width=1920"));
        assert!(args[0].contains("--height=540"));
        assert!(args[1].contains("--y=540"));
    }

    #[test]
    fn calculate_window_args_three_players() {
        let args = calculate_window_args(1920, 960, 540, 3);
        assert_eq!(args.len(), 3);
        // All should have half width
        for arg in &args {
            assert!(arg.contains("--width=960"));
        }
    }

    #[test]
    fn calculate_window_args_four_players() {
        let args = calculate_window_args(1920, 960, 540, 4);
        assert_eq!(args.len(), 4);
        // All should have half width and height
        for arg in &args {
            assert!(arg.contains("--width=960"));
            assert!(arg.contains("--height=540"));
        }
    }
}
