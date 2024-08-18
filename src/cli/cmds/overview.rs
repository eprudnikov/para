use crate::cli::cmds::area;
use crate::cli::cmds::project;
use crate::cli::context::Context;
use colored::Colorize;

pub fn run(ctx: &Context) {
    println!("{:<32}\t{:<20}", "Projects:".bold(), "Action items".bold());
    project::run(ctx);

    println!(
        "\n{:<32}\t{:<20}\t{}",
        "Areas:".bold(),
        "Action items".bold(),
        "Projects".bold()
    );
    area::run(ctx);
}

