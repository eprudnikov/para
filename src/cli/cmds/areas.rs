use crate::cli::area::Area;
use crate::cli::context::Context;
use crate::cli::read_dir;
use colored::Colorize;

pub fn run(ctx: &Context) {
    let area_names = read_dir(&ctx.areas_dir);
    for area_name in &area_names {
        let area = Area::read(&area_name, &ctx);
        println!("◦ {:<30}", area.name.bold());
    }
}