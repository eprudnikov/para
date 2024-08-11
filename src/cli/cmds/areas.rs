use crate::cli::area::Area;
use crate::cli::context::Context;
use crate::cli::read_dir;
use colored::Colorize;

pub fn run(ctx: &Context) {
    let no_projects_count = String::from(" ");
    let area_names = read_dir(&ctx.areas_dir).unwrap();
    for area_name in &area_names {
        let area = Area::read(area_name, ctx);
        let projects_count = if let Some(projects) = area.projects {
            &projects.len().to_string()
        } else {
            &no_projects_count
        };
        println!("◦ {:<30}\t{:<20}\t{}", area.name.bold(), "", projects_count);
    }
}
