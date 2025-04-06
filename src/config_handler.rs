use std::{path::PathBuf, process::exit};

use rascii_art::RenderOptions;
use serde_derive::Deserialize;

use crate::util::path_utils::get_path;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub ascii: Option<AsciiConfig>,
    pub image: Option<ImageConfig>,
}

impl Config {
    /// Creates a new `Config` instance.
    ///
    /// # Arguments
    /// * `config_path` - The path to the configuration file.
    ///
    /// # Returns
    /// * `Result<Self, toml::de::Error>` - The parsed configuration or an error.
    pub fn new(config_path: &PathBuf) -> Result<Self, toml::de::Error> {
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

        match (&config.ascii, &config.image) {
            (Some(_), Some(_)) => {
                eprintln!(
                    "Config error: Both 'ascii' and 'image' are defined. Only one must be specified."
                );
                exit(1);
            }
            (None, None) => {
                eprintln!(
                    "Config error: Neither 'ascii' nor 'image' is defined. One must be specified."
                );
                exit(1);
            }
            _ => {}
        }

        Ok(config)
    }
}

#[derive(Deserialize, Debug)]
pub struct AsciiConfig {
    pub path: PathBuf,
}

impl AsciiConfig {
    #[allow(dead_code)]
    /// Creates a new `AsciiConfig` instance.
    ///
    /// # Arguments
    /// * `path` - The path to the ASCII art file.
    ///
    /// # Returns
    /// * `AsciiConfig` - The new `AsciiConfig` instance.
    ///
    /// ```
    /// use std::{env, path::PathBuf};
    /// use symfetch::config_handler::AsciiConfig;
    ///
    /// let ascii_path = PathBuf::from("~/.config/symfetch/ascii");
    /// let ascii_config = AsciiConfig::new(ascii_path);
    /// ```
    pub fn new(path: PathBuf) -> Self {
        let path = get_path(&path);
        AsciiConfig { path }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ImageConfig {
    pub path: PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub colored: Option<bool>,
}

impl ImageConfig {
    #[allow(dead_code)]
    /// Creates a new `ImageConfig` instance.
    ///
    /// # Arguments
    /// * `path` - The path to the image file.
    ///
    /// # Returns
    /// * `ImageConfig` - The new `ImageConfig` instance.
    ///
    /// ```
    /// use std::{env, path::PathBuf};
    /// use symfetch::config_handler::ImageConfig;
    ///
    /// let image_path = PathBuf::from("~/.config/symfetch/image");
    /// let image_config = ImageConfig::new(image_path, None, None, None);
    /// ```
    pub fn new(
        path: PathBuf,
        width: Option<u32>,
        height: Option<u32>,
        colored: Option<bool>,
    ) -> Self {
        let path = get_path(&path);
        ImageConfig {
            path,
            width,
            height,
            colored,
        }
    }

    pub fn get_render_options(&self) -> RenderOptions<'static> {
        let colored = self.colored.unwrap_or(false);

        if let Some(width) = self.width {
            if let Some(height) = self.height {
                RenderOptions::new()
                    .width(width)
                    .height(height)
                    .colored(colored)
            } else {
                RenderOptions::new().width(width).colored(colored)
            }
        } else {
            RenderOptions::new()
                .width(self.width.unwrap_or(100))
                .height(self.height.unwrap_or(100))
                .colored(self.colored.unwrap_or(false))
        }
    }
}
