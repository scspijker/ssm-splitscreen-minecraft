mod logger;
mod polymc_helper;
mod minecraft_version;
mod config;

use config::Config;

use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write, BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    env,
};
use serde::{Serialize, Deserialize};
use dialog;
use dirs::home_dir;
use log::{info, error, LevelFilter};
use std::collections::HashMap;
use dialog::DialogBox;
use crate::logger::init_logger;
use crate::minecraft_version::MinecraftVersion;

fn main() -> io::Result<()> {

    let home = home_dir().expect("Could not find home directory");
    let config_directory = home.join(".ssm");
    let mut config = Config::new(&config_directory);
    init_logger(&config_directory);
    log::info!("Starting SSM Splitscreen Multiplayer");

    polymc_helper::init();

    // Prompt for Minecraft version if not set
    let minecraft_version = if config.minecraft_version.is_none() {
        let version = dialog::Input::new("Enter the Minecraft version to use for all profiles (e.g., 1.21.11):")
            .title("Minecraft Version")
            .default("1.21.11")
            .show()
            .map(|input | MinecraftVersion::validate(input).unwrap_or_default())
            .unwrap_or_default();

        config = Config {
            minecraft_version: Some(version as String),
            ..config
        };
        Config::save_config(&config, &config_directory);

    } else {
        config.minecraft_version.unwrap()
    };

    info!("Using Minecraft version: {}", minecraft_version);

    // Create player profiles
    let player_profiles = ["SSM1", "SSM2", "SSM3", "SSM4"];
    for profile in &player_profiles {
        create_polymc_profile(
            profile,
            &profiles_dir,
            &instances_dir,
            &minecraft_version,
        )?;
    }

    // Steam integration prompt
    if config.steam_add_question_answered != Some(true) && env::var("SteamDeck").is_err() {
        let desktop_entry = home.join("Desktop/SSM.desktop");
        if !desktop_entry.exists() {
            let script_path = env::current_exe()?;
            let icon_path = env::current_dir()?.join("icon.ico");

            let desktop_content = format!(
                "[Desktop Entry]\n\
                Name=SSM Splitscreen Minecraft\n\
                Exec={}\n\
                Icon={}\n\
                Type=Application\n\
                Categories=Game;\n\
                Terminal=true\n\
                StartupWMClass=konsole",
                script_path.display(),
                icon_path.display()
            );

            fs::write(&desktop_entry, desktop_content)?;
            Command::new("chmod").arg("+x").arg(&desktop_entry).status()?;
            Command::new("update-desktop-database").arg("-q").status()?;

            Dialog::info(
                "A desktop entry for this launcher has been created.\n\n\
                1. Open Steam.\n\
                2. Click 'Games' > 'Add a Non-Steam Game to My Library'.\n\
                3. Browse to \"SSM Splitscreen Minecraft\" and select it.\n\
                4. Click 'Add Selected Programs'.\n\n\
                The launcher will now appear in your Steam library.",
                None,
            )?;
        }
        config.steam_add_question_answered = Some(true);
        save_config(&config, &config_file)?;
    }

    // Get resolution and controllers
    let (width, height) = get_resolution()?;
    let (half_width, half_height) = (width / 2, height / 2);
    info!("Current resolution: {} x {}", width, height);

    let controllers = count_controllers()?;
    let built_in_enabled = is_built_in_controller_enabled()?;
    let mut players = controllers;
    let offset = if built_in_enabled && controllers > 1 { players -= 1; 1 } else { 0 };
    players = players.min(4);

    info!(
        "Controllers: {} (built-in enabled: {}), players: {}",
        controllers, built_in_enabled, players
    );

    // Window and controller arguments
    let window_args = match players {
        1 => vec!["--fullscreen".to_string()],
        2 => vec![
            format!("--width={} --height={} --x=0 --y=0", width, half_height),
            format!("--width={} --height={} --x=0 --y={}", width, half_height, half_height),
        ],
        3 => vec![
            format!("--width={} --height={} --x=0 --y=0", half_width, half_height),
            format!("--width={} --height={} --x={} --y=0", half_width, half_height, half_width),
            format!("--width={} --height={} --x=0 --y={}", half_width, half_height, half_height),
        ],
        4 => vec![
            format!("--width={} --height={} --x=0 --y=0", half_width, half_height),
            format!("--width={} --height={} --x={} --y=0", half_width, half_height, half_width),
            format!("--width={} --height={} --x=0 --y={}", half_width, half_height, half_height),
            format!("--width={} --height={} --x={} --y={}", half_width, half_height, half_width, half_height),
        ],
        _ => {
            error!("Currently only 1-4 players are supported. Requested {}, exiting..", players);
            std::process::exit(1);
        }
    };

    let controller_args: Vec<String> = (0..players)
        .map(|i| format!("--controller={}", i + offset))
        .collect();

    // Debug output
    for i in 0..players {
        info!("Player {}: {} {}", i + 1, window_args[i], controller_args[i]);
    }

    Ok(())
}

// Helper functions

fn get_resolution() -> io::Result<(u32, u32)> {
    let output = Command::new("xrandr").arg("--current").output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains('*') {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(resolution) = parts.iter().find(|s| s.contains('x')) {
                let dims: Vec<u32> = resolution.split('x').filter_map(|s| s.parse().ok()).collect();
                if dims.len() == 2 {
                    return Ok((dims[0], dims[1]));
                }
            }
        }
    }
    Ok((1920, 1080)) // Default fallback
}

fn count_controllers() -> io::Result<usize> {
    let file = File::open("/proc/bus/input/devices")?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.contains("pad") || line.contains("joystick"))
        .count())
}

fn is_built_in_controller_enabled() -> io::Result<bool> {
    let file = File::open("/proc/bus/input/devices")?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .filter_map(|line| line.ok())
        .any(|line| line.contains("Vendor=28de") && line.contains("Product=1205")))
}


