mod cli;

fn main() -> anyhow::Result<()> {
    let args = cli::build().get_matches();
    let ctx = cli::context::Context::load(&args);
    match args.subcommand() {
        Some(("overview", _)) => cli::cmds::overview::run(&ctx),
        Some(("week", _)) => cli::cmds::week::run(&ctx),
        Some(("project", args)) =>
            cli::cmds::project::run(args.get_one::<String>("name"), &ctx),
        Some(("area", args)) =>
            cli::cmds::area::run(args.get_one::<String>("name"), &ctx),
        Some(("day", _)) => cli::cmds::day::run(&ctx),
        _ => cli::cmds::overview::run(&ctx) // overview is the default command
    }
    Ok(())
}
