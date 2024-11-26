use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use chrono::{DateTime, Local};
use std::path::PathBuf;
use structopt::StructOpt;
use colored::*;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
    created_at: DateTime<Local>,
    completed_at: Option<DateTime<Local>>,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "todo",
    about = "A feature-rich command-line todo list manager",
    after_help = "Example: todo add \"Learn Rust\""
)]
enum Cli {
    #[structopt(name = "add", about = "Add a new todo item")]
    Add {
        #[structopt(help = "The todo item to add")]
        title: String,
    },
    #[structopt(name = "list", about = "List all todo items")]
    List {
        #[structopt(short, long, help = "Show only completed items")]
        completed: bool,
    },
    #[structopt(name = "complete", about = "Mark a todo item as complete")]
    Complete {
        #[structopt(help = "The id of the todo item to complete")]
        id: usize,
    },
    #[structopt(name = "delete", about = "Delete a todo item")]
    Delete {
        #[structopt(help = "The id of the todo item to delete")]
        id: usize,
    },
}

struct TodoList {
    todos: Vec<Todo>,
    file_path: PathBuf,
}

impl TodoList {
    fn new() -> io::Result<Self> {
        let file_path = PathBuf::from("todos.json");
        let todos = if file_path.exists() {
            let content = fs::read_to_string(&file_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };
        
        Ok(TodoList { todos, file_path })
    }

    fn save(&self) -> io::Result<()> {
        let content = serde_json::to_string_pretty(&self.todos)?;
        fs::write(&self.file_path, content)
    }

    fn add(&mut self, title: String) -> io::Result<()> {
        let id = self.todos.len() + 1;
        let display_title = title.clone();  // Clone it before moving
        let todo = Todo {
            id,
            title,
            completed: false,
            created_at: Local::now(),
            completed_at: None,
        };
        self.todos.push(todo);
        self.save()?;
        println!("{} Added new todo: {}", "✓".green(), display_title.cyan());
        Ok(())
    }

    fn list(&self, show_completed: bool) {
        println!("\n{}",
            if show_completed {
                "📋 Completed Tasks".green()
            } else {
                "📋 Pending Tasks".blue()
            }
        );
        println!("{}", "=".repeat(50));

        let mut found = false;
        for todo in &self.todos {
            if show_completed == todo.completed {
                found = true;
                let status = if todo.completed {
                    "✓".green()
                } else {
                    "○".yellow()
                };
                
                println!(
                    "{} [{}] {} {}",
                    status,
                    todo.id.to_string().cyan(),
                    todo.title.white(),
                    format!("(created: {})", 
                        todo.created_at.format("%Y-%m-%d %H:%M")).dimmed()
                );

                if let Some(completed_at) = todo.completed_at {
                    println!(
                        "     {} {}", 
                        "↳ completed:".green(),
                        completed_at.format("%Y-%m-%d %H:%M").to_string().dimmed()
                    );
                }
            }
        }

        if !found {
            println!("{}", 
                if show_completed {
                    "No completed tasks yet!".yellow()
                } else {
                    "No pending tasks - time to add some!".yellow()
                }
            );
        }
        println!();
    }

    fn complete(&mut self, id: usize) -> io::Result<()> {
        let title = match self.todos.iter_mut().find(|t| t.id == id) {
            Some(todo) => {
                if todo.completed {
                    println!("{} Task {} is already completed!", "!".yellow(), id);
                    return Ok(());
                }
                todo.completed = true;
                todo.completed_at = Some(Local::now());
                todo.title.clone()
            }
            None => {
                println!("{} Todo with id {} not found", "✗".red(), id);
                return Ok(());
            }
        };
        
        self.save()?;
        println!("{} Completed: {}", "✓".green(), title.cyan());
        Ok(())
    }

    fn delete(&mut self, id: usize) -> io::Result<()> {
        let title = match self.todos.iter().position(|t| t.id == id) {
            Some(index) => {
                let todo = self.todos.remove(index);
                todo.title
            }
            None => {
                println!("{} Todo with id {} not found", "✗".red(), id);
                return Ok(());
            }
        };
        
        self.save()?;
        println!("{} Deleted: {}", "✗".red(), title.cyan());
        Ok(())
    }
}

fn print_banner() {
    println!("\n{}", r#"
╭──────────────────────────────╮
│     RUST TODO MANAGER        │
╰──────────────────────────────╯"#.cyan());
}

fn main() -> io::Result<()> {
    print_banner();
    let mut todo_list = TodoList::new()?;
    let cli = Cli::from_args();

    match cli {
        Cli::Add { title } => todo_list.add(title)?,
        Cli::List { completed } => todo_list.list(completed),
        Cli::Complete { id } => todo_list.complete(id)?,
        Cli::Delete { id } => todo_list.delete(id)?,
    }

    Ok(())
}