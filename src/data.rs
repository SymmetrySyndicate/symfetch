use crate::config_handler::Config;
use crate::util::path_utils::get_path;

#[derive(Debug)]
pub struct Data {
    pub config: Config,
}

impl Data {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn render_ascii(&self) {
        if let Some(ascii_config) = &self.config.ascii {
            let ascii_content = std::fs::read_to_string(get_path(&ascii_config.path)).unwrap();
            println!("{}", ascii_content);
        } else {
            eprintln!("No ASCII config found. Skipping ASCII rendering.");
        }
    }
}
