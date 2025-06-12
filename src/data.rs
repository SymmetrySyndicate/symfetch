//! the main engine

#[cfg(feature = "image-to-ascii")]
use std::io::Cursor;

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
use std::iter;

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

    #[allow(dead_code)]
    /// If ASCII configuration is provided, write to stdout
    pub fn render_ascii(&self) {
        if let Some(ascii_config) = &self.config.ascii {
            let ascii_content = std::fs::read_to_string(get_path(&ascii_config.path)).unwrap();
            println!("{}", ascii_content);
        } else {
            eprintln!("No ASCII config found. Skipping ASCII rendering.");
        }
    }

    /// If image configuration is provided, render image as ASCII and return lines
    #[cfg(feature = "image-to-ascii")]
    pub fn render_image_as_ascii(&self) -> Option<Vec<String>> {
        if let Some(image_config) = &self.config.image {
            let image_path = get_path(&image_config.path);
            let image = ImageReader024::open(image_path)
                .ok()?
                .with_guessed_format()
                .ok()?
                .decode()
                .ok()?;
            let render_options = image_config.get_render_options();

            let mut buffer = Cursor::new(Vec::new());
            render_image(&image, &mut buffer, &render_options).ok()?;

            let rendered_ascii = String::from_utf8(buffer.into_inner()).ok()?;
            let lines: Vec<String> = rendered_ascii.lines().map(|l| l.to_string()).collect();
            Some(lines)
        } else {
            None
        }
    }

    /// If image configuration is provided, render image to stdout
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

    /// Main render function that handles all rendering logic with system info on the right side
    pub fn render(&self) {
        let mut left_side_lines = Vec::new();
        let mut has_image_content = false;

        // ASCII content
        if let Some(ascii_lines) = self.ascii_lines() {
            left_side_lines.extend(ascii_lines);
        }

        // Image as ASCII content (side-by-side support)
        #[cfg(feature = "image-to-ascii")]
        {
            if self.config.ascii.is_none() {
                if let Some(image_ascii_lines) = self.render_image_as_ascii() {
                    left_side_lines.extend(image_ascii_lines);
                    has_image_content = true;
                }
            }
        }

        // Raw image (non-side-by-side fallback)
        #[cfg(feature = "image")]
        if !has_image_content {
            eprintln!("Image rendered directly. System info will follow below.");
            self.render_image();
            has_image_content = true;
        }

        let info_lines = self.system_info_lines();

        if !left_side_lines.is_empty() {
            self.render_side_by_side(left_side_lines, info_lines);
        } else if has_image_content {
            println!(); // Spacer
            for line in info_lines {
                println!("{}", line);
            }
        } else {
            for line in info_lines {
                println!("{}", line);
            }
        }
    }

    /// Helper function to render content side-by-side with system info
    fn render_side_by_side(&self, left_lines: Vec<String>, info_lines: Vec<String>) {
        let ascii_len = left_lines.len();
        let info_len = info_lines.len();
        let max_lines = ascii_len.max(info_len);

        let left_width = left_lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);

        let padded_left: Vec<String> = left_lines
            .into_iter()
            .chain(iter::repeat(String::new()).take(max_lines - ascii_len))
            .collect();

        let padded_info: Vec<String> = info_lines
            .into_iter()
            .chain(iter::repeat(String::new()).take(max_lines - info_len))
            .collect();

        for (left, info) in padded_left.into_iter().zip(padded_info) {
            print!("{:<width$}", left, width = left_width);
            print!(" | ");
            println!("{}", info);
        }
    }
}
