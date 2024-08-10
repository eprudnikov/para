use clap::Command;

pub mod context;
pub mod project;
pub mod cmds;
pub mod area;

pub fn build() -> Command {
    Command::new("para")
        .about("CLI tool to support my PARA method")
        .subcommand(Command::new("project").about("Overview active project"))
}
