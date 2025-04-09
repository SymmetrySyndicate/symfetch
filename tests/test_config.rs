use std::path::PathBuf;

use symfetch::config_handler::Config;

#[test]
fn test_config_init() {
    let config = Config::new(&PathBuf::from("tests/only_ascii.toml")).unwrap();
    let test_ascii_path = PathBuf::from("ascii");

    let ascii_config = config.ascii.as_ref().unwrap();

    assert_eq!(&ascii_config.path, &test_ascii_path);
}
