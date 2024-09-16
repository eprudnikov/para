use crate::cli::context::Context;
use crate::cli::md;
use chrono::Datelike;
use markdown::to_mdast;
use std::fs;
use std::fs::read_to_string;
use std::path::Path;

pub struct Week {
    pub name: String,
    pub total_action_items: u16,
    pub done_action_items: u16,
    pub important_action_items: Vec<String>,
    pub interesting_action_items: Vec<String>,
}

impl Week {
    pub fn from_today(ctx: &Context, create: bool) -> Option<Self> {
        let today = chrono::offset::Local::now().date_naive();
        let week = format!("W{}", &today.iso_week().week());
        let path_as_string = format!("{}/Journaling 📔/{}/{}.md", ctx.areas_dir, today.year(), week);
        let path = Path::new(&path_as_string);
        if !path.exists() {
            if !create {
                return None;
            }
            if let Err(err) = fs::copy(&ctx.weekly_template, &path_as_string) {
                eprintln!("Failed to create the file: {}", err);
                return None;
            }
        }

        let mut important_items = Vec::new();
        let mut interesting_items = Vec::new();
        let mut total: u16 = 0;
        let mut done: u16 = 0;
        if let Ok(content) = read_to_string(path) {
            let mdast = to_mdast(&content, &markdown::ParseOptions::default());
            let binding = mdast.unwrap();
            let root_nodes = binding.children().unwrap();

            let (_, actions_start, actions_end) = md::find_goal_and_actions_positions(root_nodes);

            if let Some(start) = actions_start {
                let end = match actions_end {
                    Some(e) => e,
                    None => root_nodes.len()
                };

                (total, done, important_items, interesting_items)
                    = md::process_action_item_nodes(&root_nodes[start + 1..end]);
            }
        } else {
            eprintln!("Cannot read file {}", &path_as_string);
            return None;
        }

        Some(Week {
            name: week,
            total_action_items: total,
            done_action_items: done,
            important_action_items: important_items,
            interesting_action_items: interesting_items
        })
    }
}