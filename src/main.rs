mod cli;

fn main() -> anyhow::Result<()> {
    // let args = cli::build().get_matches();
    // match args.subcommand() {
    //     Some(("project", args)) => {
    //         cli::cmds::project::run(args)?;
    //     }
    //     _ => todo!("Not other subcommands are implemented"),
    // }
    let ctx = cli::context::Context::load();
    cli::cmds::overview::run(&ctx)?;
    Ok(())
}
