use std::fs::read_to_string;
use std::io::Read;
use std::path::Path;

use markdown::mdast::Node;
use markdown::to_mdast;

use crate::cli::context::Context;

pub struct Project {
    pub name: String,
    pub has_goal: bool,
    pub has_action_items: bool,
    pub next_action_item: Option<String>,
    pub is_complete: bool
}

impl Project {
    pub fn read(name: &str, ctx: &Context) -> Self {
        let descriptor_path_as_string = format!("{}/{}/{}.md", ctx.projects_dir, name, name);
        let descriptor_path = Path::new(&descriptor_path_as_string);
        if !descriptor_path.exists() {
            return Project {
                name: String::from(name),
                has_goal: false,
                has_action_items: false,
                next_action_item: None,
                is_complete: false
            };
        }

        let content = read_to_string(descriptor_path).unwrap();
        let mdast = to_mdast(&content, &markdown::ParseOptions::default());

        let mut is_goal_found = false;
        let mut is_action_items_found = false;
        let mut has_any_action_item = false;
        let mut next_action_item: Option<String> = None;
        for node in mdast.unwrap().children().unwrap() {
            if let Node::Heading(heading) = node {
                if let Node::Text(text) = &heading.children[0] {
                    if text.value == "Goal" {
                        is_goal_found = true;
                    }
                    if text.value == "Action items" {
                        is_action_items_found = true;
                    }
                }
            }
            if !is_action_items_found {
                continue; // the next block makes sense only inside the Action items
            }

            if let Node::List(list) = node {
                for list_child in &list.children {
                    if let Node::ListItem(list_item) = list_child {
                        let text = list_item.children[0].to_string();
                        has_any_action_item = true;
                        if text.contains("[ ]") {
                            next_action_item = Some(String::from(
                                text.replace("[ ]", "").trim()
                            ));
                            break;
                        }
                    }
                }
            }

            if next_action_item.is_some() {
                break; // found all necessary information
            }
        }

        Project {
            name: String::from(name),
            has_goal: is_goal_found,
            has_action_items: is_action_items_found,
            is_complete: has_any_action_item && next_action_item.is_none(),
            next_action_item: next_action_item,
        }
    }
}
