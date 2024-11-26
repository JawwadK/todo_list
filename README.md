# Rust Todo Manager

A command-line todo list manager with priorities, due dates, and categories.

## Quick Start

```bash
# Install
git clone [repository-url]
cd todo-list
cargo build --release

# Add tasks
cargo run -- add "Task name"                                                   # Basic task
cargo run -- add "Important task" --priority high --due "2024-12-25"          # With priority and due date
cargo run -- add "Work project" --priority high --tag work --tag urgent       # With tags

# View tasks
cargo run -- list                       # Show incomplete tasks
cargo run -- list --completed          # Show completed tasks
cargo run -- list --priority high      # Filter by priority
cargo run -- list --tag work           # Filter by tag

# Other commands
cargo run -- search "project"          # Search tasks
cargo run -- complete 1                # Complete task
cargo run -- delete 1                  # Delete task
```

## Features

- Priority levels: `high` (⚠), `medium` (◆), `low` (○)
- Due dates and categories/tags
- Persistent JSON storage
- Colored output

## Dependencies

Dependencies managed through Cargo.toml: `colored`, `serde`, `chrono`, `structopt`