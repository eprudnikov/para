use std::fs::read_to_string;
use std::path::Path;

use markdown::mdast::Node;
use markdown::mdast::List;
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

            (total_action_items, done_action_items, important_action_items)
                = process_action_item_nodes(&root_nodes[action_items_header_position + 1..end]);
        }

        Project {
            name: String::from(name),
            total_action_items,
            done_action_items,
            has_goal: is_goal_found,
            is_complete: total_action_items == done_action_items,
            important_action_items,
        }
    }
}

/// Returns the number total, done action items as well as a list of important items
fn process_action_item_nodes(nodes: &[Node]) -> (u16, u16, Vec<String>) {
    let mut total_action_items = 0;
    let mut done_action_items = 0;
    let mut important_action_items = Vec::new();
    for node in nodes {
        if let Node::List(list) = node {
            let (total, done, mut important) = process_list(list);
            total_action_items = total_action_items + total;
            done_action_items = done_action_items + done;
            important_action_items.append(&mut important);
        }
    }
    (total_action_items, done_action_items, important_action_items)
}

fn process_list(list: &List) -> (u16, u16, Vec<String>) {
    let mut total_action_items = 0;
    let mut done_action_items = 0;
    let mut important_action_items = Vec::new();

    for list_child in &list.children {
        if let Node::ListItem(list_item) = list_child {
            for list_grand_child in &list_item.children {
                if let Node::Paragraph(_paragraph) = list_grand_child {
                    let text = list_grand_child.to_string();
                    total_action_items = total_action_items + 1;
                    if text.contains("[ ]") && text.contains("❗") {
                        important_action_items.push(
                            String::from(text.replace("[ ]", "").trim())
                        );
                    }
                    if text.contains("[x]") {
                        done_action_items = done_action_items + 1;
                    }
                }

                if let Node::List(sublist) = &list_grand_child {
                    let (total, done, mut important) = process_list(sublist);
                    total_action_items = total_action_items + total;
                    done_action_items = done_action_items + done;
                    important_action_items.append(&mut important);
                }
            }
        }
    }
    (total_action_items, done_action_items, important_action_items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_empty_action_item_nodes() {
        let (total, done, important) = process_action_item_nodes(&Vec::new());
        assert_eq!(total, 0);
        assert_eq!(done, 0);
        assert_eq!(important.len(), 0);
    }

    #[test]
    fn process_non_empty_action_item_nodes() {
        let mdast = to_mdast("\
- [x] Completed task
    - [ ] Incompleted subtask
- [ ] Incompleted task
- [ ] Incompleted important task ❗
        ", &markdown::ParseOptions::default());
        let binding = mdast.unwrap();
        let root_nodes = binding.children().unwrap();

        let (total, done, important) = process_action_item_nodes(root_nodes);
        assert_eq!(total, 4);
        assert_eq!(done, 1);
        assert_eq!(important.len(), 1);
        assert_eq!(important[0], "Incompleted important task ❗");
    }
}