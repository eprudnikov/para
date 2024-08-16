use std::fs::read_to_string;
use std::path::Path;

use markdown::mdast::List;
use markdown::mdast::Node;
use markdown::to_mdast;

use crate::cli::context::Context;

pub struct Project {
    pub name: String,
    pub total_action_items: u16,
    pub done_action_items: u16,
    pub has_goal: bool,
    pub printable_action_items: Vec<String>,
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
                printable_action_items: Vec::new(),
                is_complete: false,
            };
        }

        let content = read_to_string(descriptor_path).unwrap();
        let mdast = to_mdast(&content, &markdown::ParseOptions::default());
        let binding = mdast.unwrap();
        let root_nodes = binding.children().unwrap();

        let (goal_position, actions_start, actions_end) = find_goal_and_actions_positions(root_nodes);

        let mut printable_items = Vec::new();
        let mut total: u16 = 0;
        let mut done: u16 = 0;
        if let Some(start) = actions_start {
            let end = match actions_end {
                Some(e) => e,
                None => root_nodes.len()
            };

            (total, done, printable_items) = process_action_item_nodes(&root_nodes[start + 1..end]);
        }

        Project {
            name: String::from(name),
            total_action_items: total,
            done_action_items: done,
            has_goal: goal_position.is_some(),
            is_complete: total == done,
            printable_action_items: printable_items,
        }
    }
}

fn find_goal_and_actions_positions(nodes: &[Node]) -> (Option<usize>, Option<usize>, Option<usize>) {
    let mut goal_position = None;
    let mut actions_start = None;
    let mut actions_end = None;
    for (position, node) in nodes.iter().enumerate() {
        if let Node::Heading(heading) = node {
            if heading.depth > 1 {
                continue; // skip nested headers
            }
            if actions_start.is_some() {
                // the next header after Action items is found
                actions_end = Some(position);
                break;
            }

            if let Node::Text(text) = &heading.children[0] {
                if text.value == "Goal" {
                    goal_position = Some(position);
                }
                if text.value == "Action items" {
                    actions_start = Some(position);
                }
            }
        }
    }
    (goal_position, actions_start, actions_end)
}

/// Returns the number total, done action items as well as a list of important items. The important
/// items are marked by ❗️ or ««« (in my notation, it's current task).
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
                    if text.contains("[ ]") && (text.contains("❗") || text.contains("«««")) {
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
    fn find_goal_actions_in_empty_note() {
        let (goal, actions_start, actions_end) = find_goal_and_actions_positions(&Vec::new());
        assert_eq!(goal, None);
        assert_eq!(actions_start, None);
        assert_eq!(actions_start, None);
    }
    #[test]
    fn find_goal_and_action_items_positions() {
        let mdast = to_mdast("\
# Intro
Some text

# Goal
The goal definition

# Action items
- [ ] Something to do

## Subheader to be ignored
- [ ] This is also an action item

# Additional information
Here is an extra info.
        ", &markdown::ParseOptions::default());
        let binding = mdast.unwrap();
        let root_nodes = binding.children().unwrap();

        let (goal, actions_start, actions_end) = find_goal_and_actions_positions(root_nodes);
        assert_eq!(goal, Some(2));
        assert_eq!(actions_start, Some(4));
        assert_eq!(actions_end, Some(8));
    }

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
- [ ] Current task «««
- [ ] Incompleted important task ❗
        ", &markdown::ParseOptions::default());
        let binding = mdast.unwrap();
        let root_nodes = binding.children().unwrap();

        let (total, done, important) = process_action_item_nodes(root_nodes);
        assert_eq!(total, 4);
        assert_eq!(done, 1);
        assert_eq!(important.len(), 2);
        assert_eq!(important[0], "Current task «««");
        assert_eq!(important[1], "Incompleted important task ❗");
    }
}