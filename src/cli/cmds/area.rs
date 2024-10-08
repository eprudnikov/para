use crate::cli::area::Area;
use crate::cli::context::Context;
use crate::cli::read_dir;
use colored::Colorize;

pub fn run(name: Option<&String>, ctx: &Context) {
    match name {
        Some(area_name) => {
            let area = Area::read(area_name, ctx);
            print_area(&area, ctx.verbose);
        },
        None => {
            let area_names = read_dir(&ctx.areas_dir).unwrap();
            for area_name in &area_names {
                let area = Area::read(area_name, ctx);
                print_area(&area, ctx.verbose);
            }
        }
    }
}

fn print_area(area: &Area, verbose: bool) {
    let no_items = " ";
    let action_item_count = if area.total_action_items > 0 {
        format!("{}/{}", area.done_action_items, area.total_action_items)
    } else {
        String::from(no_items)
    };
    let projects_count = if let Some(projects) = &area.projects {
        projects.len().to_string()
    } else {
        String::from(no_items)
    };
    println!("◦ {:<30}\t{:<20}\t{}", area.name.bold(), action_item_count, projects_count);
    for item in &area.important_action_items {
        if item.len() > 100 { // TODO extract it to a common function
            println!("  ・ {}…", item[..100].to_string());
        } else {
            println!("  ・ {}", item);
        }
    }

    if verbose {
        for item in &area.interesting_action_items {
            if item.len() > 100 {
                println!("  ・ {}…", item[..100].to_string());
            } else {
                println!("  ・ {}", item);
            }
        }

        if let Some(projects) = &area.projects {
            for project in projects {
                println!("  ‣ {}", project);
            }
        }
    }
}
