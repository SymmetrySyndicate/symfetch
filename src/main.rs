use clap::{arg, command, value_parser};
use config_handler::Config;
use data::Data;
use std::{env, path::PathBuf};

mod config_handler;
mod data;
mod util;

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

    if data.config.ascii.is_some() {
        data.render_ascii();
    }
}
