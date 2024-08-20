use crate::cli::area::Area;
use crate::cli::context::Context;
use crate::cli::project::Project;
use crate::cli::read_dir;
use chrono::Datelike;
use colored::Colorize;
use std::fs;
use std::path::Path;
use crate::cli::week::Week;

pub fn run(ctx: &Context) {
    let today = chrono::offset::Local::now().date_naive();
    let path_as_string = format!("{}/Journaling 📔/{}/{}.md", ctx.areas_dir, today.year(), &today);
    let daily_note_path = Path::new(&path_as_string);
    if daily_note_path.exists() {
        println!("The daily note exists.");
    } else {
        println!("Create type {}", path_as_string);
        if let Err(err) = fs::copy(&ctx.daily_template, daily_note_path) {
            eprintln!("Failed to create the file: {}", err);
            return;
        }
    }

    println!("\nPlease consider the following action items:");
    print_project_action_items(ctx);
    print_area_action_items(ctx);
    print_week_action_items(ctx);
}

fn print_project_action_items(ctx: &Context) {
    let project_names = read_dir(&ctx.projects_dir).unwrap();
    for project_name in &project_names {
        let project = Project::read(project_name, ctx);
        print_action_items(&project.name, &project.printable_action_items);
    }
}

fn print_area_action_items(ctx: &Context) {
    let area_names = read_dir(&ctx.areas_dir).unwrap();
    for area_name in &area_names {
        let area = Area::read(area_name, ctx);
        print_action_items(&area.name, &area.printable_action_items);
    }
}

fn print_week_action_items(ctx: &Context) {
    match Week::from_today(ctx) {
        None => println!("The weekly note doesn't exist. Please consider creating it."),
        Some(week) => {
            print_action_items(&week.name, &week.printable_action_items);
        }
    }
}

fn print_action_items(name: &str, action_items: &Vec<String>) {
    if !action_items.is_empty() {
        println!("◦ {:<30}", name.bold());
        for item in action_items {
            if item.len() > 100 {
                println!("  ・{}…", item[..100].to_string())
            } else {
                println!("  ・{}", item)
            }
        }
    }
}

