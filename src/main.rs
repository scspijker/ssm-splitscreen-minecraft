mod config;
mod logger;
mod minecraft_version;
mod polymc_helper;
mod setup;
mod system;
mod window_layout;

use crate::logger::init_logger;
use crate::setup::{
    perform_initial_setup, prompt_minecraft_version, save_configuration, should_perform_setup,
};
use crate::system::{count_controllers, get_resolution, is_built_in_controller_enabled};
use crate::window_layout::calculate_window_args;
use config::Config;
use dirs::home_dir;
use log::info;

fn main() -> std::io::Result<()> {
    let home = home_dir().expect("Could not find home directory");
    let config_directory = home.join(".ssm");
    let config = Config::new(&config_directory);
    init_logger(&config_directory);
    info!("Starting SSM Splitscreen Multiplayer");

    polymc_helper::init();

    // Prompt for Minecraft version if not set
    let minecraft_version = prompt_minecraft_version(&config);
    info!("Using Minecraft version: {}", minecraft_version);

    // Perform initial setup if needed
    if should_perform_setup(&config) {
        perform_initial_setup();
    }

    // Save configuration
    save_configuration(&minecraft_version, &config_directory);

    // Get system information
    let (width, height) = get_resolution()?;
    let (half_width, half_height) = (width / 2, height / 2);
    info!("Current resolution: {} x {}", width, height);

    let controllers = count_controllers()?;
    let built_in_enabled = is_built_in_controller_enabled()?;
    let offset = if built_in_enabled && controllers > 1 {
        1
    } else {
        0
    };
    let players = 4.min(controllers - offset); // Max 4 players supported

    info!(
        "Controllers: {} (built-in enabled: {}), players: {}",
        controllers, built_in_enabled, players
    );

    // Calculate window and controller arguments
    let window_args = calculate_window_args(width, half_width, half_height, players);
    let controller_args: Vec<String> = (0..players)
        .map(|i| format!("--controller={}", i + offset))
        .collect();

    // Output configuration
    for i in 0..players {
        log::debug!(
            "Player {}: {} {}",
            i + 1,
            window_args[i],
            controller_args[i]
        );
    }

    // TODO: Setup / update PolyMC instances with these arguments
    // TODO: Launch PolyMC instances configured above.

    Ok(())
}
