use std::{env, path::Path, path::PathBuf, process::exit};

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub ascii: AsciiConfig,
}

#[derive(Deserialize, Debug)]
pub struct AsciiConfig {
    pub path: PathBuf,
}

pub fn parse_config(config_path: &PathBuf) -> Result<Config, toml::de::Error> {
    let contents = match std::fs::read_to_string(config_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Failed to read config file: {}", err);
            exit(1);
        }
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to parse config file: {}", err);
            exit(1);
        }
    };

    Ok(config)
}

pub fn get_ascii(path: &Path) -> Result<String, std::io::Error> {
    let path = get_ascii_path(path);
    std::fs::read_to_string(&path)
}

pub fn get_ascii_path(path: &Path) -> PathBuf {
    let xdg_config_home = PathBuf::from(env::var("XDG_CONFIG_HOME").expect("HOME not set"));

    if path.to_str().unwrap().contains("~/.config/") {
        let path_str = path.to_str().unwrap();
        let sub_path = PathBuf::from(path_str.replace("~/.config/", ""));
        xdg_config_home.join(&sub_path)
    } else {
        path.to_path_buf()
    }
}
