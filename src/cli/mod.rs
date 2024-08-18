use io::Error;
use std::{fs, io};
use clap::Command;

pub mod area;
pub mod cmds;
pub mod context;
pub mod project;
mod md;

pub fn build() -> Command {
    Command::new("para")
        .about("CLI tool to support my PARA method")
        .subcommand(Command::new("overview").about("List all active projects and all areas"))
        .subcommand(Command::new("project")
            .alias("projects")
            .about("Show details of a specific project or list all of them."))
        .subcommand(Command::new("area")
            .alias("areas")
            .about("Show details of a specific area or list all of them."))
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

