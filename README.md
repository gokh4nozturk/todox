# Todo List CLI

A simple and efficient command-line todo list manager written in Rust.

## Features

- ✨ Add new tasks
- 📝 List all tasks
- ✅ Mark tasks as completed
- 🗑️ Remove tasks
- 💾 Automatic saving to JSON file
- 🎨 Colorful terminal output

## Installation

You can install the todo list CLI using cargo:

```bash
cargo install to-do-list-cli
```

## Usage

### Add a new task
```bash
todo add "Learn Rust programming"
```

### List all tasks
```bash
todo list
```

### Mark a task as done
```bash
todo done 1
```

### Remove a task
```bash
todo remove 1
```

## Development

To build from source:

```bash
git clone https://github.com/gokh4nozturk/to-do-list-cli
cd to-do-list-cli
cargo build --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 