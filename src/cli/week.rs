use std::fs;
use crate::cli::context::Context;
use crate::cli::md;
use chrono::Datelike;
use markdown::mdast::Node;
use markdown::to_mdast;
use std::fs::read_to_string;
use std::path::Path;

pub struct Week {
    pub name: String,
    pub printable_action_items: Vec<String>,
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

        let mut printable_items = Vec::new();
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

                for node in &root_nodes[start + 1..end] {
                    if let Node::List(list) = node {
                        for list_child in &list.children {
                            if let Node::ListItem(list_item) = list_child {
                                for list_grand_child in &list_item.children {
                                    if let Node::Paragraph(_paragraph) = list_grand_child {
                                        let text = list_grand_child.to_string();
                                        if text.contains("[ ]") {
                                            printable_items.push(
                                                String::from(text.replace("[ ]", "").trim())
                                            );
                                        }
                                    }

                                    if let Node::List(_sublist) = &list_grand_child {
                                        eprintln!("Cannot process nested lists yet.")
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            eprintln!("Cannot read file {}", &path_as_string);
            return None;
        }

        Some(Week {
            name: week,
            printable_action_items: printable_items,
        })
    }
}