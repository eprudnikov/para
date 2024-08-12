use super::read_dir;
use crate::cli::context::Context;
use markdown::mdast::Node;
use markdown::to_mdast;
use std::fs::read_to_string;

pub struct Area {
    pub name: String,
    pub projects: Option<Vec<String>>,
    pub total_action_items: Option<u16>,
    pub done_action_items: Option<u16>
}

impl Area {
    pub fn read(name: &str, ctx: &Context) -> Self {
        let projects_path = format!("{}/{}/Projects", ctx.areas_dir, name);
        let projects: Option<Vec<String>> = if let Ok(projects) = read_dir(&projects_path) {
            Some(projects)
        } else {
            None
        };

        let mut total_action_items = 0;
        let mut done_action_items = 0;
        let descriptor_path = format!("{}/{}/{}.md", ctx.areas_dir, name, name);
        if let Ok(content) = read_to_string(descriptor_path) {
            let mdast = to_mdast(&content, &markdown::ParseOptions::default());

            let mut is_action_items_found = false;
            let mut action_items_header_position: usize = 0;
            let mut end_action_items_header: usize = 0;

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
                                if text.contains("[x]") {
                                    done_action_items = done_action_items + 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        Area {
            name: String::from(name),
            projects,
            total_action_items: if total_action_items > 0 {
                Some(total_action_items)
            } else {
                None
            },
            done_action_items: if done_action_items > 0 {
                Some(done_action_items)
            } else {
                None
            }
        }
    }
}
