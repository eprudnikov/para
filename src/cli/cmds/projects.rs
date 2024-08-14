use colored::Colorize;

use crate::cli::context::Context;
use crate::cli::project::Project;
use crate::cli::read_dir;

pub fn run(ctx: &Context) {
    let project_names = read_dir(&ctx.projects_dir).unwrap();
    for project_name in &project_names {
        let project = Project::read(project_name, ctx);
        print_project(&project);
    }
}

fn print_project(project: &Project) {
    let status: &str = if project.is_complete {
        "🤩"
    } else if !project.has_goal || project.total_action_items == 0 {
        "🤔"
    } else {
        " "
    };
    println!(
        "◦ {:<30}\t{}/{} {}",
        project.name.bold(),
        project.done_action_items,
        project.total_action_items,
        status
    );
    for item in &project.important_action_items {
        if item.len() > 100 {
            println!("  ・{}…", item[..100].to_string())
        } else {
            println!("  ・{}", item)
        }
    }
    if !project.has_goal {
        println!("  ・{}", "The project has no defined goal".red())
    }
    if project.total_action_items == 0 {
        println!("  ・ {}", "The project has no action items.".red())
    }
}
