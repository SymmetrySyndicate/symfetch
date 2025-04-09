//! the main engine
use std::io;

use image::io::Reader as ImageReader;
use rascii_art::render_image;

use crate::config_handler::Config;
use crate::util::path_utils::get_path;

/// holds information about config (+ system data)
#[derive(Debug)]
pub struct Data {
    /// parsed information from config
    pub config: Config,
}

impl Data {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// If ASCII configuration is provided, write to stdout
    pub fn render_ascii(&self) {
        if let Some(ascii_config) = &self.config.ascii {
            let ascii_content = std::fs::read_to_string(get_path(&ascii_config.path)).unwrap();
            println!("{}", ascii_content);
        } else {
            eprintln!("No ASCII config found. Skipping ASCII rendering.");
        }
    }

    /// If image configuration is provided, render image to stdout
    ///
    /// NOTE: uses [`rascii_art::render_image`]
    pub fn render_image(&self) {
        if let Some(image_config) = &self.config.image {
            let image_path = get_path(&image_config.path);
            let image = ImageReader::open(image_path)
                .unwrap()
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            let render_options = image_config.get_render_options();

            let stdout = io::stdout();
            let mut buffer = stdout.lock();

            render_image(&image, &mut buffer, &render_options).unwrap();
        } else {
            eprintln!("No image config found. Skipping image rendering.");
        }
    }
}
