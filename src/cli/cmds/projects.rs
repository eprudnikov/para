use std::fs;

use anyhow::Result;
use colored::Colorize;

use crate::cli::context::Context;
use crate::cli::project::Project;

pub fn run(ctx: &Context) -> Result<()> {
    match fs::read_dir(&ctx.projects_dir) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            let mut project_names = Vec::new();
            for path in paths {
                let dir_entity = path?;
                let file_name = dir_entity.file_name();
                let project_name = file_name.to_str().unwrap();

                if dir_entity.file_type().is_ok_and(|f| f.is_dir())
                    && !project_name.starts_with(".")
                {
                    project_names.push(String::from(project_name));
                }
            }

            for project_name in &project_names {
                let project = Project::read(&project_name, &ctx);
                let status: &str = if project.is_complete {
                    "🤩"
                } else if !project.has_goal || !project.has_action_items {
                    "🤔"
                } else {
                    " "
                };
                println!("◦ {:<30}\t{}/{} {}",
                        project.name.bold(),
                        project.done_action_items,
                        project.total_action_items,
                        status
                    );
                if !project.has_goal {
                    println!("\t・{}", "The project has no defined goal".red())
                }
                if !project.has_action_items {
                    println!("\t・ {}", "The project has no action items.".red())
                }
            }
        }
    }
    Ok(())
}
