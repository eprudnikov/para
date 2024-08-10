use std::fs;

use anyhow::Result;
use colored::Colorize;

use crate::cli::context::Context;
use crate::cli::project::Project;
use crate::cli::read_dir;

pub fn run(ctx: &Context) {
    let project_names = read_dir(&ctx.projects_dir);
    for project_name in &project_names {
        let project = Project::read(&project_name, &ctx);
        print_project(&project);
    }
}

fn print_project(project: &Project) {
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
