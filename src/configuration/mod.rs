use serde::{Deserialize, Serialize};
use std::fs::metadata;
use std::fs::File;
use std::string::ToString;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(default)]
    pub observed_directories: Vec<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            observed_directories: vec!["/boot".to_string(), "/usr/bin".to_string()],
        }
    }
}

impl Configuration {
    pub fn from_defaut_locations() -> Configuration {
        if metadata("/etc/sakerhet/config.yml").is_ok() {
            return Configuration::from_file("/etc/sakerhet/config.yml");
        } else if metadata("config.yml").is_ok() {
            return Configuration::from_file("config.yml");
        }
        Configuration::default()
    }

    pub fn from_file(config_file: &str) -> Configuration {
        let file_handle = File::open(config_file);
        if file_handle.is_ok() {
            let read_configuration: Configuration =
                serde_yaml::from_reader(file_handle.unwrap()).unwrap();
            return read_configuration;
        }
        Configuration::default()
    }

    pub fn from_yaml(configuration: &str) -> Configuration {
        let read_configuration: Configuration = serde_yaml::from_str(configuration).unwrap();
        read_configuration
    }
}
