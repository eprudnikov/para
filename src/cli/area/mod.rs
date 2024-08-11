use crate::cli::context::Context;

use super::read_dir;

pub struct Area {
    pub name: String,
    pub projects: Option<Vec<String>>,
}

impl Area {
    pub fn read(name: &str, ctx: &Context) -> Self {
        let projects_path = format!("{}/{}/Projects", ctx.areas_dir, name);
        let projects: Option<Vec<String>> = if let Ok(projects) = read_dir(&projects_path) {
            Some(projects)
        } else {
            None
        };
        Area {
            name: String::from(name),
            projects
        }
    }
}
