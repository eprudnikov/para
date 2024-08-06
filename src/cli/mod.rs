use clap::Command;

pub mod context;
pub mod projects;
pub mod cmds;

pub fn build() -> Command {
    Command::new("para")
        .about("CLI tool to support my PARA method")
        .subcommand(Command::new("projects").about("Overview active projects"))
}
