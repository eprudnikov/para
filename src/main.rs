mod cli;

fn main() -> anyhow::Result<()> {
    let args = cli::build().get_matches();

    match args.subcommand() {
        Some(("projects", args)) => {
            cli::cmds::projects::run(args)?;
        }
        _ => todo!("Not other subcommands are implemented"),
    }

    Ok(())
}
