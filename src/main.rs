mod cli;

fn main() -> anyhow::Result<()> {
    let args = cli::build().get_matches();
    let ctx = cli::context::Context::load();
    match args.subcommand() {
        Some(("overview", _)) => cli::cmds::overview::run(&ctx),
        Some(("project", _)) => cli::cmds::project::run(&ctx),
        Some(("area", _)) => cli::cmds::area::run(&ctx),
        Some(("day", _)) => todo!("The day command is not implemented yet."),
        _ => cli::cmds::overview::run(&ctx) // overview is the default command
    }
    Ok(())
}
