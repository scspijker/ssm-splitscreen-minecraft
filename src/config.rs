use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use dirs::home_dir;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub steam_add_question_answered: Option<bool>,
    pub minecraft_version: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            steam_add_question_answered: Some(false),
            minecraft_version: None,
        }
    }
}

impl Config {
    pub fn new(config_directory: &PathBuf) -> Self {
        // Create the config directory if it doesn't exist
        fs::create_dir_all(&config_directory).expect(format!("Could not create config directory {}", config_directory.display()).as_str());

        Self::read_config(&config_directory)
    }

    fn config_file_path(config_directory: &PathBuf) -> PathBuf {
        config_directory.join("config.json")
    }

    fn read_config(config_directory: &PathBuf) -> Self {
        File::open(Self::config_file_path(config_directory))
            .map(|f| serde_json::from_reader::<File, Config>(f))
            .unwrap_or(Ok(Config::default())).unwrap()
    }

    pub fn save_config(config: &Config, config_directory: &PathBuf) {
        let file_path = Self::config_file_path(config_directory);
        File::create(&file_path)
            .expect(&format!("Failed to create config file at {}", file_path.display()))
            .write_all(serde_json::to_string_pretty(&config).unwrap().as_bytes())
            .expect(&format!("Failed to write config file at {}", file_path.display()));
    }
}