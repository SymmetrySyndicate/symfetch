use clap::{arg, command, value_parser};
use config_handler::Config;
use data::Data;
use std::{env, path::PathBuf};

mod config_handler;
mod data;
mod system_info;
mod util;

#[allow(unused_variables)]
fn main() {
    let matches = command!()
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let config_path = matches
        .get_one::<PathBuf>("config")
        .cloned()
        .unwrap_or_else(|| {
            let home = env::var("HOME").expect("Environment variable $HOME not set");
            PathBuf::from(format!("{}/.config/symfetch.toml", home))
        });

    let config = Config::new(&config_path).unwrap();
    let data = Data::new(config);

    // let ascii_lines = data.ascii_lines().unwrap_or_else(|| vec![]);
    let sysinfo_lines = data.system_info_lines();

    // Define the image block size (width and height)
    let image_width = 40; // characters wide
    let image_height = 10; // lines tall

    // Determine the total number of lines to print
    let total_lines = image_height.max(sysinfo_lines.len());

    for i in 0..total_lines {
        let mut line = String::new();

        // Add ASCII art if within image bounds
        // if i < ascii_lines.len() && i < image_height {
        if i < image_height {
            // let ascii_line = &ascii_lines[i];

            // Truncate ASCII line to fit within image block
            // let truncated_line: String = ascii_line
            //     .chars()
            //     .take(image_width)
            //     .collect();

            // Center the truncated ASCII art within the image block
            // let padding = (image_width - truncated_line.chars().count()) / 2;
            // line.push_str(&" ".repeat(padding));
            // line.push_str(&truncated_line);

            // Pad to full image width
            while line.chars().count() < image_width {
                line.push(' ');
            }
        } else {
            // Empty space for image block
            line.push_str(&" ".repeat(image_width));
        }

        // Add separator
        line.push_str("   ");

        // Add system info
        if i < sysinfo_lines.len() {
            line.push_str(&sysinfo_lines[i]);
        }

        println!("{}", line);
    }

    if data.config.ascii.is_some() {
        data.render_ascii();
    }

    // if let Some(image_config) = &data.config.image {
    //     #[cfg(feature = "image-to-ascii")]
    //     if image_config.as_ascii.is_some() {
    //         data.render_image_as_ascii();
    //     }

    //     #[cfg(feature = "image")]
    //     {
    //         data.render_image();
    //     }
    // }
}
