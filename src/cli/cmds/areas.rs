use crate::cli::area::Area;
use crate::cli::context::Context;
use colored::Colorize;
use std::fs;

pub fn run(ctx: &Context) -> anyhow::Result<()> {
    match fs::read_dir(&ctx.areas_dir) {
        Err(why) => println!("❗️ {:?}", why.kind()),
        Ok(paths) => {
            let mut area_names = Vec::new();
            for path in paths {
                let dir_entity = path?;
                let file_name = dir_entity.file_name();
                let project_name = file_name.to_str().unwrap();

                if dir_entity.file_type().is_ok_and(|f| f.is_dir())
                    && !project_name.starts_with(".")
                {
                    area_names.push(String::from(project_name));
                }
            }

            for area_name in &area_names {
                let area = Area::read(&area_name, &ctx);
                println!("◦ {:<30}", area.name.bold());
            }
        }
    }
    Ok(())
}