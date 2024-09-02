use clap::ArgMatches;
use homedir::my_home;
use serde_derive::Deserialize;
use std::fs;
use std::process::exit;

#[derive(Deserialize)]
struct Data {
    // It's okay to reuse the Context struct as config model.
    config: Context,
}

#[derive(Deserialize)]
pub struct Context {
    pub projects_dir: String,
    pub areas_dir: String,
    pub daily_template: String,
    pub weekly_template: String,
    #[serde(skip_deserializing)]
    pub verbose: bool,
}

impl Context {
    pub fn load(arg_matches: &ArgMatches) -> Self {
        let mut config_path = my_home().unwrap().unwrap();
        config_path.push(".config/para/para.toml");

        let config_content = match fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Cannot read the user context {}: {}", config_path.to_str().unwrap(), e);
                exit(1);
            }
        };

        let mut data: Data = match toml::from_str(&config_content) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Cannot parse the user context {}: {}", config_path.to_str().unwrap(), e);
                exit(1);
            }
        };
        data.config.verbose = arg_matches.get_flag("verbose");
        data.config
    }
}

