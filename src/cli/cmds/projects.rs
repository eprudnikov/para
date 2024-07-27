use anyhow::Result;
use clap::ArgMatches;
use std::fs;

pub fn run(args: &ArgMatches) -> Result<()> {
    match fs::read_dir("/Users/eprudnikov/second-brain/1. Projects") {
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
                    println!("\t{}", project_name);
                }
            }
        }
    }
    Ok(())
}
