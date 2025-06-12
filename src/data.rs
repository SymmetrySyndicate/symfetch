//! the main engine

#[cfg(feature = "image-to-ascii")]
use std::io;

#[cfg(feature = "image-to-ascii")]
use rascii_art::render_image;

#[cfg(feature = "image-to-ascii")]
use image::io::Reader as ImageReader024;

#[cfg(feature = "image")]
use image_025::ImageReader as ImageReader025;

#[cfg(feature = "image")]
use viuer::print;

use crate::config_handler::Config;
use crate::system_info::SystemInfo;
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

    /// If image configuration is provided, render image as ascii to stdout
    ///
    /// NOTE: uses [`rascii_art::render_image`]
    #[cfg(feature = "image-to-ascii")]
    pub fn render_image_as_ascii(&self) {
        if let Some(image_config) = &self.config.image {
            let image_path = get_path(&image_config.path);
            let image = ImageReader024::open(image_path)
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

    /// If image configuration is provided, render image to stdout
    ///
    /// NOTE: uses [`viuer::print`]
    #[cfg(feature = "image")]
    #[allow(dead_code)]
    pub fn render_image(&self) {
        if let Some(image_config) = &self.config.image {
            let viuer_config = image_config.get_viuer_config();
            let image_path = get_path(&image_config.path);
            let image = ImageReader025::open(image_path)
                .unwrap()
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            print(&image, &viuer_config).expect("Image printing failed.");
        }
    }

    pub fn ascii_lines(&self) -> Option<Vec<String>> {
        if let Some(ascii_config) = &self.config.ascii {
            let ascii_content = std::fs::read_to_string(get_path(&ascii_config.path)).ok()?;
            Some(ascii_content.lines().map(|l| l.to_string()).collect())
        } else {
            None
        }
    }

    pub fn system_info_lines(&self) -> Vec<String> {
        let system_info = SystemInfo::new();
        system_info.render_lines()
    }
}
