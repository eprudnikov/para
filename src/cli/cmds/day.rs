use crate::cli::context::Context;
use chrono::Datelike;
use std::fs;
use std::path::Path;

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
}

