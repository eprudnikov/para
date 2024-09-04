use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;
use markdown::to_mdast;

use crate::cli::context::Context;
use crate::cli::md;

pub struct Project {
    pub name: String,
    pub total_action_items: u16,
    pub done_action_items: u16,
    pub has_goal: bool,
    pub important_action_items: Vec<String>,
    pub interesting_action_items: Vec<String>,
    pub is_complete: bool,
}

impl Project {
    pub fn read(name: &str, ctx: &Context) -> Self {
        let project_path = format!("{}/{}", ctx.projects_dir, name);
        let project_path = Path::new(&project_path);
        if !project_path.is_dir() {
            eprintln!("There is no project '{}'", name);
            exit(1);
        }

        let descriptor_path_as_string = format!("{}/{}/{}.md", ctx.projects_dir, name, name);
        let descriptor_path = Path::new(&descriptor_path_as_string);
        if !descriptor_path.exists() {
            return Project {
                name: String::from(name),
                total_action_items: 0,
                done_action_items: 0,
                has_goal: false,
                important_action_items: Vec::new(),
                interesting_action_items: Vec::new(),
                is_complete: false,
            };
        }

        let content = read_to_string(descriptor_path).unwrap();
        let mdast = to_mdast(&content, &markdown::ParseOptions::default());
        let binding = mdast.unwrap();
        let root_nodes = binding.children().unwrap();

        let (goal_position, actions_start, actions_end) = md::find_goal_and_actions_positions(root_nodes);

        let mut important_items = Vec::new();
        let mut interesting_items = Vec::new();
        let mut total: u16 = 0;
        let mut done: u16 = 0;
        if let Some(start) = actions_start {
            let end = match actions_end {
                Some(e) => e,
                None => root_nodes.len()
            };

            (total, done, important_items, interesting_items)
                = md::process_action_item_nodes(&root_nodes[start + 1..end]);
        }

        Project {
            name: String::from(name),
            total_action_items: total,
            done_action_items: done,
            has_goal: goal_position.is_some(),
            is_complete: total == done,
            important_action_items: important_items,
            interesting_action_items: interesting_items
        }
    }
}