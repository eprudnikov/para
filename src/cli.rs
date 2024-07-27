use clap::Command;

pub fn build() -> Command {
    Command::new("para")
        .about("CLI tool to support my PARA method")
        .subcommand(Command::new("projects").about("Overview active projects"))
}
