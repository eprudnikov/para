use clap::Command;
use std::fs;
use std::process::exit;

pub mod context;
pub mod project;
pub mod cmds;
pub mod area;

pub fn build() -> Command {
    Command::new("para")
        .about("CLI tool to support my PARA method")
        .subcommand(Command::new("project").about("Overview active project"))
}

fn read_dir(path: &str) -> Vec<String>{
    match fs::read_dir(path) {
        Err(why) => {
            println!("❗️ {:?}", why.kind());
            exit(1);
        },
        Ok(paths) => {
            let mut result = Vec::new();
            for p in paths {
                let dir_entity = p.unwrap();
                let file_name = dir_entity.file_name();
                let dir_name = file_name.to_str().unwrap();

                if dir_entity.file_type().is_ok_and(|f| f.is_dir())
                    && !dir_name.starts_with(".")
                {
                    result.push(String::from(dir_name));
                }
            }
            result
        }
    }
}