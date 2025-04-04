use std::{env, path::PathBuf};

use symfetch::config_handler::get_ascii_path;

#[test]
fn test_ascii_path() {
    let ascii_path = get_ascii_path(&PathBuf::from("~/.config/symfetch/ascii"));

    let xdg_config_home = PathBuf::from(env::var("XDG_CONFIG_HOME").unwrap());
    let path = xdg_config_home.join("symfetch/ascii");

    assert_eq!(ascii_path, path)
}
