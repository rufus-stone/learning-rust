use directories::ProjectDirs;
use simple_logger::SimpleLogger;

use std::io::Write;

use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG: &str = r#"
# Blah blah
name = "Alice"
age = 27
food = "APPLE"
"#;

#[derive(Debug, Serialize, Deserialize)]
enum Food {
    APPLE,
    PEAR,
    BERRY,
}

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    name: Option<String>,
    age: Option<u8>,
    food: Option<Food>,
}

fn read_config(file_path: &std::path::Path) -> Option<MyConfig> {
    if file_path.exists() {
        // Read the file contents
        let contents = std::fs::read_to_string(file_path).expect("Failed to read data from file!");

        let config: MyConfig = toml::from_str(&contents).unwrap();

        Some(config)
    } else {
        None
    }
}

fn create_default_config_file_if_missing(
    config_path: &std::path::Path,
) -> Result<(), std::io::Error> {
    // Is there no config file?
    if !config_path.exists() {
        log::warn!(
            "No config file found... Creating one at: {}",
            config_path.display()
        );

        // Create any missing dirs
        let prefix = config_path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();

        // Create a new config file
        let mut config_file = match std::fs::File::create(&config_path) {
            Err(why) => panic!(
                "Failed to create file at {}: {}",
                config_path.display(),
                why
            ),
            Ok(file) => file,
        };

        // Write the default config to the file
        config_file.write_all(DEFAULT_CONFIG.as_bytes())
    } else {
        Ok(())
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();

    // Determine where the config file should be found
    let project_dirs = ProjectDirs::from("com", "somecompany", "dirtest").unwrap();
    let config_dir = project_dirs.config_dir();
    let config_path = config_dir.join("config.toml");
    log::info!("{:?}", config_path);

    create_default_config_file_if_missing(&config_path).expect("Failed to create config file!");

    // Read the settings from the file...
    let settings = read_config(&config_path).expect("Failed to read config file!");
    log::info!("{:?}", settings);

    // Create a new config
    let new_config = MyConfig {
        name: Some("Bob".to_owned()),
        age: Some(99),
        food: Some(Food::BERRY),
    };

    // Convert to TOML
    let toml = toml::to_string(&new_config).unwrap();
    log::info!("{}", toml);
}
