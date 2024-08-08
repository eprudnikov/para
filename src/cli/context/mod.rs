use std::fs;
use std::process::exit;

use homedir::my_home;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Data {
    config: Context
}

#[derive(Deserialize)]
pub struct Context {
    pub base_dir: String,
    pub projects_dir: String,
}

impl Context {
    pub fn load() -> Self {
        let mut config_path = my_home().unwrap().unwrap();
        config_path.push(".config/para/para.toml");

        let config_content = match fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Cannot read the user context {}: {}", config_path.to_str().unwrap(), e);
                exit(1);
            }
        };

        let data: Data = match toml::from_str(&config_content) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Cannot parse the user context {}: {}", config_path.to_str().unwrap(), e);
                exit(1);
            }
        };
        data.config
    }
}

