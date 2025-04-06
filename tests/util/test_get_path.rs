use std::{env, path::PathBuf};

use symfetch::config_handler::AsciiConfig;
use symfetch::util::path_utils::get_path;

#[test]
fn test_ascii_path() {
    let xdg_config_home = PathBuf::from(env::var("XDG_CONFIG_HOME").unwrap());
    let path = xdg_config_home.join("symfetch/ascii");

    let ascii_config = AsciiConfig::new(PathBuf::from("~/.config/symfetch/ascii"));

    assert_eq!(get_path(&ascii_config.path), path)
}
