//! the main engine

#[cfg(feature = "image-to-ascii")]
use image as image024;

#[cfg(feature = "image-to-ascii")]
use image024::io::Reader as ImageReader024;

#[cfg(feature = "image-to-ascii")]
use rascii_art::render_image;

#[cfg(feature = "image")]
use image_025 as image;

#[cfg(feature = "image")]
use image::GenericImageView;

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

    /// Returns ASCII art lines if configured
    pub fn ascii_lines(&self) -> Option<Vec<String>> {
        self.config.ascii.as_ref().and_then(|ascii_config| {
            std::fs::read_to_string(get_path(&ascii_config.path))
                .ok()
                .map(|content| {
                    let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
                    let max_width = lines
                        .iter()
                        .map(|line| line.chars().count())
                        .max()
                        .unwrap_or(0);
                    lines
                        .into_iter()
                        .map(|line| format!("{line:<max_width$}"))
                        .collect()
                })
        })
    }

    /// Main render function that handles all rendering logic with system info on the right side
    pub fn render(&self) {
        let mut left_side_lines = self.ascii_lines().unwrap_or_default();

        #[cfg(feature = "image-to-ascii")]
        {
            if self.config.ascii.is_none() {
                if let Some(image_ascii_lines) = self.image_as_ascii_vec() {
                    left_side_lines.extend(image_ascii_lines);
                }
            }
        }

        #[cfg(feature = "image")]
        #[allow(clippy::collapsible_if)]
        {
            if self.config.ascii.is_none() && left_side_lines.is_empty() {
                if let Some(image_ansi_lines) = self.image_as_ansi_vec() {
                    left_side_lines.extend(image_ansi_lines);
                }
            }
        }

        let system_info = SystemInfo::new();
        let info_lines = system_info.as_vec();

        if !left_side_lines.is_empty() {
            self.render_side_by_side(&left_side_lines, &info_lines);
        } else {
            for line in info_lines {
                println!("{line}");
            }
        }
    }

    /// Helper function to render content side-by-side with system info
    fn render_side_by_side(&self, left_lines: &[String], info_lines: &[String]) {
        let max_lines = left_lines.len().max(info_lines.len());

        let empty = String::new();
        let padded_left = left_lines
            .iter()
            .chain(iter::repeat(&empty))
            .take(max_lines);
        let padded_info = info_lines
            .iter()
            .chain(iter::repeat(&empty))
            .take(max_lines);

        let left_width = self.left_width(left_lines);

        for (left_line, info_line) in padded_left.zip(padded_info) {
            if left_line.is_empty() {
                print!("{:<width$}", "", width = left_width);
            } else {
                print!("{left_line}");
            }
            print!(" | ");
            println!("{info_line}");
        }
    }

    /// Helper to determine the width for left-side padding
    fn left_width(&self, left_lines: &[String]) -> usize {
        if let Some(image_config) = &self.config.image {
            image_config.width.unwrap_or_else(|| {
                left_lines
                    .first()
                    .map(|line| line.chars().count() as u32)
                    .unwrap_or(0)
            }) as usize
        } else {
            left_lines
                .first()
                .map(|line| line.chars().count())
                .unwrap_or(0)
        }
    }

    #[cfg(feature = "image-to-ascii")]
    /// If image configuration is provided, render image as ASCII and return lines
    pub fn image_as_ascii_vec(&self) -> Option<Vec<String>> {
        use std::io::Cursor;

        let image_config = self.config.image.as_ref()?;
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
        Some(rendered_ascii.lines().map(|l| l.to_string()).collect())
    }

    #[cfg(feature = "image")]
    fn image_as_ansi_vec(&self) -> Option<Vec<String>> {
        use termimage::ops;
        use termimage::{AnsiOutputFormat, Options};

        let image_config = self.config.image.as_ref()?;
        let image_path = get_path(&image_config.path);
        let image_path_str = image_path.to_string_lossy().to_string();
        let size = (
            image_config.width.unwrap_or(40),
            image_config.height.unwrap_or(20),
        );
        let opts = Options {
            image: (image_path_str.clone(), image_path.clone()),
            size,
            preserve_aspect: true,
            ansi_out: Some(AnsiOutputFormat::Truecolor),
        };
        let format = ops::guess_format(&opts.image).ok()?;
        let img = ops::load_image(&opts.image, format).ok()?;
        let img_s = ops::image_resized_size(img.dimensions(), opts.size, opts.preserve_aspect);
        let resized = ops::resize_image(&img, img_s);
        let mut buf = Vec::new();
        ops::write_ansi_truecolor(&mut buf, &resized);
        let rendered = String::from_utf8(buf).ok()?;
        Some(rendered.lines().map(|l| l.trim_end().to_string()).collect())
    }
}
