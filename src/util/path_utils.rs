use std::{
    env,
    path::{Path, PathBuf},
};

/// Returns the path to the ASCII art file.
///
/// If the path starts with `~/.config/`, it will be replaced with the user's home directory.
/// Otherwise, the path will be returned as is.
///
/// # Arguments
/// * `path` - The path to the ASCII art file.
///
/// # Returns
/// * `PathBuf` - The path to the ASCII art file.
///
/// ```
/// use std::{env, path::PathBuf};
/// use symfetch::util::path_utils::get_path;
///
/// let ascii_path = PathBuf::from("~/.config/symfetch/ascii");
/// let ascii_path = get_path(&ascii_path);
///
/// let home = PathBuf::from(env::var("HOME").unwrap());
/// let path = home.join(".config/symfetch/ascii");
/// assert_eq!(ascii_path, path);
/// ```
pub fn get_path(path: &Path) -> PathBuf {
    let mut config_home =
        PathBuf::from(env::var("HOME").expect("Environment variable HOME not set"));
    config_home = config_home.join(".config");

    if path.to_str().unwrap().contains("~/.config/") {
        let path_str = path.to_str().unwrap();
        let sub_path = PathBuf::from(path_str.replace("~/.config/", ""));
        config_home.join(&sub_path)
    } else {
        path.to_path_buf()
    }
}
