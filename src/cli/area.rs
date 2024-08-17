use super::{md, read_dir};
use crate::cli::context::Context;
use markdown::to_mdast;
use std::fs::read_to_string;

pub struct Area {
    pub name: String,
    pub projects: Option<Vec<String>>,
    pub total_action_items: u16,
    pub done_action_items: u16,
    pub printable_action_items: Vec<String>,
}

impl Area {
    pub fn read(name: &str, ctx: &Context) -> Self {
        let projects_path = format!("{}/{}/Projects", ctx.areas_dir, name);
        let projects: Option<Vec<String>> = if let Ok(projects) = read_dir(&projects_path) {
            Some(projects)
        } else {
            None
        };

        let mut printable_items = Vec::new();
        let mut total: u16 = 0;
        let mut done: u16 = 0;

        let descriptor_path = format!("{}/{}/{}.md", ctx.areas_dir, name, name);
        if let Ok(content) = read_to_string(descriptor_path) {
            let mdast = to_mdast(&content, &markdown::ParseOptions::default());
            let binding = mdast.unwrap();
            let root_nodes = binding.children().unwrap();

            let (_, actions_start, actions_end) = md::find_goal_and_actions_positions(root_nodes);
            if let Some(start) = actions_start {
                let end = match actions_end {
                    Some(e) => e,
                    None => root_nodes.len()
                };

                (total, done, printable_items) = md::process_action_item_nodes(&root_nodes[start + 1..end]);
            }
        }

        Area {
            name: String::from(name),
            projects,
            total_action_items: total,
            done_action_items: done,
            printable_action_items: printable_items
        }
    }
}
