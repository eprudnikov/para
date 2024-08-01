use anyhow::Result;
use clap::ArgMatches;
use std::fs;
use std::path::Path;

pub fn run(args: &ArgMatches) -> Result<()> {
    let projects_directory = "/Users/eprudnikov/second-brain/1. Projects";
    match fs::read_dir(projects_directory) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            println!("Projects:");
            for path in paths {
                let dir_entity = path.unwrap();
                let file_name = dir_entity.file_name();
                let project_name = file_name.to_str().unwrap();

                if dir_entity.file_type().is_ok_and(|f| f.is_dir())
                    && !project_name.starts_with(".")
                {
                    let ok_symbol = if has_project_description(project_name) {
                        "👍"
                    } else {
                        "👎"
                    };
                    println!("\t{} {}", project_name, ok_symbol);
                }
            }
        }
    }
    Ok(())
}

fn has_project_description(project_name: &str) -> bool {
    let descriptor_path = format!(
        "/Users/eprudnikov/second-brain/1. Projects/{}/{}.md", project_name, project_name
    );
    Path::new(&descriptor_path).exists()
}
