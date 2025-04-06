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

/// Returns the path to the ASCII art file.
///
/// If the path starts with `~/.config/`, it will be replaced with the user's home directory.
/// Otherwise, the path will be returned as is.
///
/// # Arguments
/// * `path` - The path to the ASCII art file.
///
/// # Returns
/// * `PathBuf` - The path to the ASCII art file.
///
/// ```
/// use std::{env, path::PathBuf};
/// use symfetch::config_handler::get_ascii_path;
///
/// let ascii_path = get_ascii_path(&PathBuf::from("~/.config/symfetch/ascii"));
///
/// let home = PathBuf::from(env::var("HOME").unwrap());
/// let path = home.join(".config/symfetch/ascii");
/// assert_eq!(ascii_path, path);
/// ```
pub fn get_ascii_path(path: &Path) -> PathBuf {
    let mut config_home =
        PathBuf::from(env::var("HOME").expect("Environment variable HOME not set"));
    config_home = config_home.join(".config");

    if path.to_str().unwrap().contains("~/.config/") {
        let path_str = path.to_str().unwrap();
        let sub_path = PathBuf::from(path_str.replace("~/.config/", ""));
        config_home.join(&sub_path)
    } else {
        path.to_path_buf()
    }
}
