//! parse and process configuration
use std::{path::PathBuf, process::exit};

#[cfg(feature = "image-to-ascii")]
use rascii_art::RenderOptions;

use serde_derive::Deserialize;

use crate::util::path_utils::get_path;

/// core struct to store data parsed from the configuration file
///
/// NOTE: Notice how `ascii` and `image` are an [`Option`] i.e. one can provide
/// either or None
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
    ///
    /// # Examples
    /// Let's say your configuration file looks like this
    ///
    /// ```toml
    /// [ascii]
    /// path="ascii"
    /// ```
    /// i.e. you want to show some ASCII art saved in some file at path `ascii`
    ///
    /// You can access the path as follows
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use symfetch::config_handler::Config;
    ///
    /// let config = Config::new(&PathBuf::from("tests/only_ascii.toml")).unwrap();
    /// let ascii_config = &config.ascii.as_ref().unwrap();
    ///
    /// assert_eq!(&ascii_config.path, &PathBuf::from("tests/ascii"));
    /// ```
    pub fn new(config_path: &PathBuf) -> Result<Self, toml::de::Error> {
        let contents = match std::fs::read_to_string(config_path) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Failed to read config file: {err}");
                exit(1);
            }
        };

        let config: Config = match toml::from_str(&contents) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Failed to parse config file: {err}");
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

/// store parsed information from the "\[ascii\]" table
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
    pub fn new(path: PathBuf) -> Self {
        let path = get_path(&path);
        AsciiConfig { path }
    }
}

/// store parsed information from the "\[image\]" table
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ImageConfig {
    /// The path to the image file.
    pub path: PathBuf,

    /// width parameter for [`RenderOptions`]
    pub width: Option<u32>,

    /// height parameter for [`RenderOptions`]
    pub height: Option<u32>,

    /// colored parameter for [`RenderOptions`]
    pub colored: Option<bool>,

    /// boolean parameter controlling ASCII conversion
    pub as_ascii: Option<bool>,
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
    /// let image_config = ImageConfig::new(image_path, None, None, None, None);
    /// ```
    pub fn new(
        path: PathBuf,
        width: Option<u32>,
        height: Option<u32>,
        colored: Option<bool>,
        as_ascii: Option<bool>,
    ) -> Self {
        let path = get_path(&path);
        ImageConfig {
            path,
            width,
            height,
            colored,
            as_ascii,
        }
    }

    /// create [`RenderOptions`] for calling `rascii_art::render_image`
    ///
    /// if while creating a [`ImageConfig`] instance, the `width`, `height` and `colored`
    /// parameters are not provided, the default values of 100, 100 and false are used.
    #[cfg(feature = "image-to-ascii")]
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
