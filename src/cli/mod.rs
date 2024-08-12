use clap::Command;
use io::Error;
use std::{fs, io};

pub mod area;
pub mod cmds;
pub mod context;
pub mod project;

pub fn build() -> Command {
    Command::new("para")
        .about("CLI tool to support my PARA method")
        .subcommand(Command::new("projects").about("Overview active projects"))
        .subcommand(Command::new("areas").about("Overview all areas"))
        .subcommand(Command::new("day").about("Init today's note in the Journal"))
}

fn read_dir(path: &str) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();
    for p in fs::read_dir(path)? {
        let dir_entity = p.unwrap();
        let file_name = dir_entity.file_name();
        let dir_name = file_name.to_str().unwrap();

        if dir_entity.file_type().is_ok_and(|f| f.is_dir()) && !dir_name.starts_with(".") {
            result.push(String::from(dir_name));
        }
    }
    result.sort();
    Ok(result)
}

