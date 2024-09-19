use colored::Colorize;
use crate::cli::context::Context;
use crate::cli::week::Week;

pub fn run(ctx: &Context) {
    match Week::from_today(ctx, false) {
        Some(week) => print(&week, ctx),
        None => println!("The week note doesn't exist")
    }
}

fn print(week: &Week, ctx: &Context) {
    println!("{:<32}\t{}/{}", week.name.bold(), week.done_action_items, week.total_action_items);
    for item in &week.important_action_items {
        if item.len() > 100 {
            println!("・{}…", item[..100].to_string())
        } else {
            println!("・{}", item)
        }
    }
    if ctx.verbose {
        for item in &week.interesting_action_items {
            if item.len() > 100 {
                println!("・{}…", item[..100].to_string())
            } else {
                println!("・{}", item)
            }
        }
    }
}