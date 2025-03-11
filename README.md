# Todox

A simple and efficient command-line todo list manager written in Rust.

## Features

- ✨ Add new tasks
- 📝 List all tasks
- ✅ Mark tasks as completed
- 🗑️ Remove tasks
- 💾 Automatic saving to JSON file
- 🎨 Colorful terminal output

## Installation

You can install todox using cargo:

```bash
cargo install todox
```

## Usage

### Add a new task
```bash
todox add "Learn Rust programming"
```

### List all tasks
```bash
todox list
```

### Mark a task as done
```bash
todox done 1
```

### Remove a task
```bash
todox remove 1
```

## Development

To build from source:

```bash
git clone https://github.com/gokh4nozturk/todox
cd todox
cargo build --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 