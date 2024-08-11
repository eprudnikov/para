use crate::cli::cmds::areas;
use crate::cli::cmds::projects;
use crate::cli::context::Context;
use colored::Colorize;

pub fn run(ctx: &Context) -> anyhow::Result<()> {
    println!("{:<32}\t{:<20}", "Projects:".bold(), "Action items".bold());
    projects::run(ctx);

    println!(
        "\n{:<32}\t{:<20}\t{}",
        "Areas:".bold(),
        "Action items".bold(),
        "Projects".bold()
    );
    areas::run(ctx);
    Ok(())
}

