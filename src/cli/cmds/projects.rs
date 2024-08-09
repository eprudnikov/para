use std::fs;

use anyhow::Result;
use colored::Colorize;

use crate::cli::context::Context;
use crate::cli::projects::Project;

pub fn run(config: Context) -> Result<()> {
    match fs::read_dir(&config.projects_dir) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                let dir_entity = path.unwrap();
                let file_name = dir_entity.file_name();
                let project_name = file_name.to_str().unwrap();

                if dir_entity.file_type().is_ok_and(|f| f.is_dir())
                    && !project_name.starts_with(".")
                {
                    let project = Project::read(&project_name, &config);
                    let status: &str = if project.is_complete {
                        "🤩"
                    } else if !project.has_goal || !project.has_action_items {
                        "🤔"
                    } else {
                        ""
                    };
                    println!("◦ {} {}\t{}/{}", project.name.bold(), status,
                             project.done_action_items, project.total_action_items);
                    if !project.has_goal {
                        println!("\t・{}", "The project has no defined goal".red())
                    }
                    if !project.has_action_items {
                        println!("\t・ {}", "The project has no action items.".red())
                    }
                    // break;
                }
            }
        }
    }
    Ok(())
}


