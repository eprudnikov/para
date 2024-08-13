use crate::cli::area::Area;
use crate::cli::context::Context;
use crate::cli::read_dir;
use colored::Colorize;

pub fn run(ctx: &Context) {
    let no_items = " ";
    let area_names = read_dir(&ctx.areas_dir).unwrap();
    for area_name in &area_names {
        let area = Area::read(area_name, ctx);
        let action_item_count = if let Some(total) = area.total_action_items {
            let done = if let Some(d) = area.done_action_items {
                d
            } else {
                0
            };
            format!("{}/{}", done, &total)
        } else {
            String::from(no_items)
        };
        let projects_count = if let Some(projects) = area.projects {
            projects.len().to_string()
        } else {
            String::from(no_items)
        };
        println!("◦ {:<30}\t{:<20}\t{}", area.name.bold(), action_item_count, projects_count);
        for item in &area.important_action_items {
            println!("  ・{}", item)
        }
    }
}
