# Rust Todo CLI

A simple command-line todo list manager with a colorful interface.

## Setup

```bash
cargo build --release
```

## Usage

```bash
# Add a todo
cargo run -- add "Learn Rust"

# List todos
cargo run -- list
cargo run -- list --completed

# Complete/Delete todos
cargo run -- complete <id>
cargo run -- delete <id>
```

## Features

- ✨ Colorful interface
- 📝 Add, complete, and delete todos
- 💾 Automatic JSON storage
- 🕒 Timestamp tracking

## License

MIT - See [LICENSE](LICENSE) for details