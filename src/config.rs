use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub initial_setup: Option<bool>,
    pub minecraft_version: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            initial_setup: Some(false),
            minecraft_version: None,
        }
    }
}

impl Config {
    pub fn new(config_directory: &PathBuf) -> Self {
        // Create the config directory if it doesn't exist
        fs::create_dir_all(config_directory).unwrap_or_else(|_| {
            panic!(
                "Could not create config directory {}",
                config_directory.display()
            )
        });

        Self::read_config(config_directory)
    }

    fn config_file_path(config_directory: &Path) -> PathBuf {
        config_directory.join("config.json")
    }

    fn read_config(config_directory: &Path) -> Self {
        File::open(Self::config_file_path(config_directory))
            .map(serde_json::from_reader::<File, Config>)
            .unwrap_or(Ok(Config::default()))
            .unwrap()
    }

    pub fn save_config(config: &Config, config_directory: &Path) {
        let file_path = Self::config_file_path(config_directory);
        File::create(&file_path)
            .unwrap_or_else(|_| panic!("Failed to create config file at {}", file_path.display()))
            .write_all(serde_json::to_string_pretty(&config).unwrap().as_bytes())
            .unwrap_or_else(|_| panic!("Failed to write config file at {}", file_path.display()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn default_config_has_initial_setup_false_and_no_version() {
        let config = Config::default();
        assert_eq!(config.initial_setup, Some(false));
        assert_eq!(config.minecraft_version, None);
    }

    #[test]
    fn new_creates_config_directory() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("test_config");
        assert!(!config_dir.exists());

        let _ = Config::new(&config_dir);

        assert!(config_dir.exists());
    }

    #[test]
    fn new_returns_default_when_no_config_file_exists() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().to_path_buf();

        let config = Config::new(&config_dir);

        assert_eq!(config.initial_setup, Some(false));
        assert_eq!(config.minecraft_version, None);
    }

    #[test]
    fn save_and_load_preserves_config_values() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path();

        let original = Config {
            initial_setup: Some(true),
            minecraft_version: Some("1.21.1".to_string()),
        };

        Config::save_config(&original, config_dir);
        let loaded = Config::new(&temp_dir.path().to_path_buf());

        assert_eq!(loaded.initial_setup, original.initial_setup);
        assert_eq!(loaded.minecraft_version, original.minecraft_version);
    }

    #[test]
    fn config_file_path_joins_correctly() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path();

        let file_path = Config::config_file_path(config_dir);

        assert_eq!(file_path.file_name().unwrap(), "config.json");
        assert!(file_path.starts_with(config_dir));
    }

    #[test]
    fn save_creates_config_json_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path();

        let config = Config {
            initial_setup: Some(true),
            minecraft_version: Some("1.20.4".to_string()),
        };

        Config::save_config(&config, config_dir);

        let file_path = Config::config_file_path(config_dir);
        assert!(file_path.exists());

        let contents = fs::read_to_string(&file_path).unwrap();
        assert!(contents.contains("1.20.4"));
        assert!(contents.contains("true"));
    }

    #[test]
    fn save_config_with_none_values() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path();

        let config = Config {
            initial_setup: None,
            minecraft_version: None,
        };

        Config::save_config(&config, config_dir);
        let loaded = Config::new(&temp_dir.path().to_path_buf());

        assert_eq!(loaded.initial_setup, None);
        assert_eq!(loaded.minecraft_version, None);
    }

    #[test]
    fn save_config_with_mixed_none_and_some_values() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path();

        let config = Config {
            initial_setup: Some(false),
            minecraft_version: Some("1.19.2".to_string()),
        };

        Config::save_config(&config, config_dir);
        let loaded = Config::new(&temp_dir.path().to_path_buf());

        assert_eq!(loaded.initial_setup, Some(false));
        assert_eq!(loaded.minecraft_version, Some("1.19.2".to_string()));
    }

    #[test]
    fn config_is_serializable_and_deserializable() {
        let config = Config {
            initial_setup: Some(true),
            minecraft_version: Some("1.21".to_string()),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.initial_setup, config.initial_setup);
        assert_eq!(deserialized.minecraft_version, config.minecraft_version);
    }

    #[test]
    fn config_is_cloneable() {
        let original = Config {
            initial_setup: Some(true),
            minecraft_version: Some("1.21".to_string()),
        };

        let cloned = original.clone();

        assert_eq!(cloned.initial_setup, original.initial_setup);
        assert_eq!(cloned.minecraft_version, original.minecraft_version);
    }
}
