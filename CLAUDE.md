# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Run Commands

```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Run the CLI
cargo run

# Run with arguments
cargo run -- overview
cargo run -- project -n <project-name>
cargo run -- area -n <area-name>
cargo run -- day
cargo run -- week

# Run tests
cargo test

# Run a single test
cargo test <test_name>
# Example: cargo test find_goal_and_action_items_positions
```

## Architecture

This is a Rust CLI tool for managing notes using the PARA method (Projects, Areas, Resources, Archives). It parses markdown files to track action items and display progress.

### Configuration

The tool reads configuration from `~/.config/para/para.toml`:
```toml
[config]
projects_dir = "/path/to/projects"
areas_dir = "/path/to/areas"
daily_template = "/path/to/daily_template.md"
weekly_template = "/path/to/weekly_template.md"
```

### Module Structure

- `src/main.rs` - Entry point, routes subcommands to handlers
- `src/cli/mod.rs` - CLI definition using clap, subcommand setup
- `src/cli/context.rs` - Configuration loading from TOML file
- `src/cli/project.rs` - Project struct and markdown parsing for project notes
- `src/cli/area.rs` - Area struct and parsing, including nested projects
- `src/cli/week.rs` - Weekly note management (stored in `{areas_dir}/Journaling/{year}/W{week}.md`)
- `src/cli/day.rs` - Daily note path generation (stored in `{areas_dir}/Journaling/{year}/{date}.md`)
- `src/cli/md.rs` - Core markdown parsing logic for extracting action items
- `src/cli/cmds/` - Command handlers for each subcommand

### Markdown Conventions

The tool expects specific markdown structure in notes:
- `# Goal` - Section for project goals
- `# Action items` - Section containing task lists
- `[ ]` - Incomplete task
- `[x]` - Completed task
- `[ ] ~~text~~` - Ignored task (counted as done)
- `[ ] task <<<` or `[ ] task ❗` - Important/current task
- `[ ] task 😍` - Interesting task

### Key Functions

`md::find_goal_and_actions_positions()` - Locates Goal and Action items sections in markdown AST
`md::process_action_item_nodes()` - Extracts task counts and categorizes tasks by markers
