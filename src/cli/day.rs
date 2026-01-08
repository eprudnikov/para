use crate::cli::context::Context;
use chrono::{Datelike, Duration};

// The struct represents daily notes.
pub struct Day {
    pub name: String,
    pub touched_projects: Vec<String>,
}

impl Day {
    /// Offset from the current day.
    pub fn get_path(ctx: &Context, offset: i64) -> String {
        let day = chrono::offset::Local::now() + Duration::days(offset);
        format!(
            "{}/Journaling/{}/{}.md",
            ctx.areas_dir,
            day.year(),
            &day.date_naive()
        )
    }
}

