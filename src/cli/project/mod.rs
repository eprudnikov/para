use std::fs::read_to_string;
use std::path::Path;

use markdown::mdast::Node;
use markdown::to_mdast;

use crate::cli::context::Context;

pub struct Project {
    pub name: String,
    pub total_action_items: u16,
    pub done_action_items: u16,
    pub has_goal: bool,
    pub important_action_items: Vec<String>,
    pub is_complete: bool,
}

impl Project {
    pub fn read(name: &str, ctx: &Context) -> Self {
        let descriptor_path_as_string = format!("{}/{}/{}.md", ctx.projects_dir, name, name);
        let descriptor_path = Path::new(&descriptor_path_as_string);
        if !descriptor_path.exists() {
            return Project {
                name: String::from(name),
                total_action_items: 0,
                done_action_items: 0,
                has_goal: false,
                important_action_items: Vec::new(),
                is_complete: false,
            };
        }

        let content = read_to_string(descriptor_path).unwrap();
        let mdast = to_mdast(&content, &markdown::ParseOptions::default());

        let mut is_goal_found = false;
        let mut is_action_items_found = false;
        let mut important_action_items = Vec::new();
        let mut action_items_header_position: usize = 0;
        let mut end_action_items_header: usize = 0;
        let mut total_action_items: u16 = 0;
        let mut done_action_items: u16 = 0;

        let binding = mdast.unwrap();
        let root_nodes = binding.children().unwrap();
        for (position, node) in root_nodes.iter().enumerate() {
            if let Node::Heading(heading) = node {
                if heading.depth > 1 {
                    continue; // skip nested headers
                }
                if is_action_items_found {
                    // the next header after Action items is found
                    end_action_items_header = position;
                    break;
                }

                if let Node::Text(text) = &heading.children[0] {
                    if text.value == "Goal" {
                        is_goal_found = true;
                    }
                    if text.value == "Action items" {
                        is_action_items_found = true;
                        action_items_header_position = position;
                    }
                }
            }
        }

        if is_action_items_found {
            let end = if end_action_items_header == 0 {
                root_nodes.len()
            } else {
                end_action_items_header
            };

            for node in &root_nodes[action_items_header_position + 1..end] {
                if let Node::List(list) = node {
                    for list_child in &list.children {
                        if let Node::ListItem(list_item) = list_child {
                            let text = list_item.children[0].to_string();
                            total_action_items = total_action_items + 1;
                            if text.contains("[ ]") && text.contains("❗️") {
                                important_action_items.push(
                                    String::from(text.replace("[ ]", "").trim())
                                );
                            }
                            if text.contains("[x]") {
                                done_action_items = done_action_items + 1;
                            }
                        }
                    }
                }
            }
        }

        Project {
            name: String::from(name),
            total_action_items,
            done_action_items,
            has_goal: is_goal_found,
            is_complete: total_action_items == done_action_items,
            important_action_items
        }
    }
}
