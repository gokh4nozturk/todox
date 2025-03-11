use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Local};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
    created_at: DateTime<Local>,
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI todo list manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo
    Add {
        /// The todo title
        title: String,
    },
    /// List all todos
    List,
    /// Complete a todo
    Done {
        /// The todo ID
        id: usize,
    },
    /// Remove a todo
    Remove {
        /// The todo ID
        id: usize,
    },
}

fn main() {
    let cli = Cli::parse();
    let todo_file = get_todo_file();
    let mut todos = load_todos(&todo_file);

    match cli.command {
        Commands::Add { title } => {
            let id = todos.len() + 1;
            let todo = Todo {
                id,
                title,
                completed: false,
                created_at: Local::now(),
            };
            todos.push(todo);
            save_todos(&todo_file, &todos);
            println!("{}", "Todo added successfully!".green());
        }
        Commands::List => {
            if todos.is_empty() {
                println!("{}", "No todos found!".yellow());
                return;
            }
            for todo in todos {
                let status = if todo.completed {
                    "✓".green()
                } else {
                    "✗".red()
                };
                println!(
                    "{} [{}] {} ({})",
                    todo.id,
                    status,
                    todo.title,
                    todo.created_at.format("%Y-%m-%d %H:%M")
                );
            }
        }
        Commands::Done { id } => {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = true;
                save_todos(&todo_file, &todos);
                println!("{}", "Todo marked as done!".green());
            } else {
                println!("{}", "Todo not found!".red());
            }
        }
        Commands::Remove { id } => {
            if let Some(pos) = todos.iter().position(|t| t.id == id) {
                todos.remove(pos);
                save_todos(&todo_file, &todos);
                println!("{}", "Todo removed successfully!".green());
            } else {
                println!("{}", "Todo not found!".red());
            }
        }
    }
}

fn get_todo_file() -> PathBuf {
    let mut home = dirs::home_dir().expect("Could not find home directory");
    home.push(".todos.json");
    home
}

fn load_todos(file: &PathBuf) -> Vec<Todo> {
    if !file.exists() {
        return Vec::new();
    }
    let contents = fs::read_to_string(file).expect("Could not read todo file");
    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
}

fn save_todos(file: &PathBuf, todos: &[Todo]) {
    let contents = serde_json::to_string_pretty(todos).expect("Could not serialize todos");
    fs::write(file, contents).expect("Could not write todo file");
}
