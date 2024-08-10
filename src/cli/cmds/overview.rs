use colored::Colorize;
use crate::cli::context::Context;
use crate::cli::cmds::projects;
use crate::cli::cmds::areas;

pub fn run(ctx: &Context) -> anyhow::Result<()> {
    println!("{}", "Projects:".bold());
    projects::run(ctx);

    println!("\n{}", "Areas:".bold());
    areas::run(ctx);
    Ok(())
}