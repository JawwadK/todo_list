use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use chrono::{DateTime, Local};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
    created_at: DateTime<Local>,
    completed_at: Option<DateTime<Local>>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "todo", about = "A command-line todo list manager")]
enum Cli {
    #[structopt(name = "add")]
    Add {
        #[structopt(help = "The todo item to add")]
        title: String,
    },
    #[structopt(name = "list")]
    List {
        #[structopt(short, long, help = "Show only completed items")]
        completed: bool,
    },
    #[structopt(name = "complete")]
    Complete {
        #[structopt(help = "The id of the todo item to complete")]
        id: usize,
    },
    #[structopt(name = "delete")]
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
        let todo = Todo {
            id,
            title,
            completed: false,
            created_at: Local::now(),
            completed_at: None,
        };
        self.todos.push(todo);
        self.save()
    }

    fn list(&self, show_completed: bool) {
        for todo in &self.todos {
            if show_completed == todo.completed {
                println!(
                    "[{}] {}: {} (created: {})",
                    if todo.completed { "✓" } else { " " },
                    todo.id,
                    todo.title,
                    todo.created_at.format("%Y-%m-%d %H:%M")
                );
            }
        }
    }

    fn complete(&mut self, id: usize) -> io::Result<()> {
        let title = match self.todos.iter_mut().find(|t| t.id == id) {
            Some(todo) => {
                todo.completed = true;
                todo.completed_at = Some(Local::now());
                todo.title.clone()
            }
            None => {
                println!("Todo with id {} not found", id);
                return Ok(());
            }
        };
        
        self.save()?;
        println!("Completed todo: {}", title);
        Ok(())
    }

    fn delete(&mut self, id: usize) -> io::Result<()> {
        let title = match self.todos.iter().position(|t| t.id == id) {
            Some(index) => {
                let todo = self.todos.remove(index);
                todo.title
            }
            None => {
                println!("Todo with id {} not found", id);
                return Ok(());
            }
        };
        
        self.save()?;
        println!("Deleted todo: {}", title);
        Ok(())
    }
}

fn main() -> io::Result<()> {
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