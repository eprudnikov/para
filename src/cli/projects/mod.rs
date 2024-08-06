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
}

impl Project {
    pub fn read(name: &str, ctx: &Context) -> Self {
        let descriptor_path_as_string = format!("{}/{}/{}.md", ctx.project_directory, name, name);
        let descriptor_path = Path::new(&descriptor_path_as_string);
        if !descriptor_path.exists() {
            return Project {
                name: String::from(name),
                has_goal: false,
                has_action_items: false,
                next_action_item: None,
            }
        }

        let content = read_to_string(descriptor_path).unwrap();
        let mdast = to_mdast(&content, &markdown::ParseOptions::default());

        let mut is_goal_found = false;
        let mut is_action_items_found = false;
        for node in mdast.unwrap().children().unwrap() {
            if let Node::Heading(heading) = node {
                // println!("\t{:?}", heading);
                if let Node::Text(text) = &heading.children[0] {
                    if text.value == "Goal" {
                        is_goal_found = true;
                    }
                    if text.value == "Action items" {
                        is_action_items_found = true;
                    }
                }
            }
        }

        Project {
            name: String::from(name),
            has_goal: is_goal_found,
            has_action_items: is_action_items_found,
            next_action_item: None,
        }
    }
}

// fn read_file(path: &Path) -> String {
//     let mut file = File::open(path).unwrap();
//     let mut content = String::new();
//     file.read_to_string(&mut content).unwrap();
//     content
// }
