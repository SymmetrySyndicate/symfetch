use clap::{arg, command, value_parser};
use std::{env, path::PathBuf};

mod config_handler;

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

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        let cfg = config_handler::parse_config(config_path).unwrap();
        let ascii = config_handler::get_ascii(&cfg.ascii.path).unwrap();
        println!("{}", ascii);
    }
}
