use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use chrono::{DateTime, Local, NaiveDateTime};
use std::path::PathBuf;
use structopt::StructOpt;
use colored::*;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
    created_at: DateTime<Local>,
    completed_at: Option<DateTime<Local>>,
    priority: Priority,
    due_date: Option<NaiveDateTime>,
    categories: Vec<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "todo", about = "A feature-rich todo list manager")]
enum Cli {
    #[structopt(name = "add")]
    Add {
        #[structopt(help = "The todo item to add")]
        title: String,
        #[structopt(long = "priority", help = "Priority level (high/medium/low)")]
        priority: Option<String>,
        #[structopt(long = "due", help = "Due date (YYYY-MM-DD)")]
        due: Option<String>,
        #[structopt(long = "tag", help = "Categories (can be used multiple times)", multiple = true)]
        tags: Vec<String>,
    },
    #[structopt(name = "list")]
    List {
        #[structopt(long = "completed", help = "Show only completed items")]
        completed: bool,
        #[structopt(long = "priority", help = "Filter by priority")]
        priority: Option<String>,
        #[structopt(long = "tag", help = "Filter by category")]
        tag: Option<String>,
    },
    #[structopt(name = "search")]
    Search {
        query: String,
    },
    Complete {
        id: usize,
    },
    Delete {
        id: usize,
    },
}
impl Todo {
    fn new(title: String, priority_str: Option<String>, due_date_str: Option<String>, categories: Vec<String>) -> Self {
        let priority = match priority_str.as_deref() {
            Some("high") => Priority::High,
            Some("medium") => Priority::Medium,
            _ => Priority::Low,
        };

        let due_date = due_date_str.and_then(|date_str| {
            NaiveDateTime::parse_from_str(&format!("{} 23:59:59", date_str), "%Y-%m-%d %H:%M:%S").ok()
        });

        Todo {
            id: 0, // Will be set when adding to list
            title,
            completed: false,
            created_at: Local::now(),
            completed_at: None,
            priority,
            due_date,
            categories,
        }
    }

    fn format_priority(&self) -> ColoredString {
        match self.priority {
            Priority::High => "⚠ HIGH".red(),
            Priority::Medium => "◆ MED".yellow(),
            Priority::Low => "○ LOW".green(),
        }
    }
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

    fn add(&mut self, title: String, priority: Option<String>, due: Option<String>, categories: Vec<String>) -> io::Result<()> {
        let mut todo = Todo::new(title.clone(), priority, due, categories);
        todo.id = self.todos.len() + 1;
        self.todos.push(todo);
        self.save()?;
        println!("{} Added new todo: {}", "✓".green(), title.cyan());
        Ok(())
    }

    fn list(&self, show_completed: bool, priority_filter: Option<String>, category_filter: Option<String>) {
        println!("\n{}", "📋 Tasks".blue());
        println!("{}", "=".repeat(50));

        let mut found = false;
        for todo in &self.todos {
            if show_completed == todo.completed {
                // Apply filters
                if let Some(ref priority) = priority_filter {
                    let todo_priority = match todo.priority {
                        Priority::High => "high",
                        Priority::Medium => "medium",
                        Priority::Low => "low",
                    };
                    if priority != todo_priority {
                        continue;
                    }
                }

                if let Some(ref category) = category_filter {
                    if !todo.categories.contains(&category.to_string()) {
                        continue;
                    }
                }

                found = true;
                self.display_todo(todo);
            }
        }

        if !found {
            println!("{}", "No matching tasks found!".yellow());
        }
        println!();
    }

    fn search(&self, query: &str) {
        println!("\n{} '{}'", "🔍 Search results for".blue(), query.cyan());
        println!("{}", "=".repeat(50));

        let mut found = false;
        for todo in &self.todos {
            if todo.title.to_lowercase().contains(&query.to_lowercase()) {
                found = true;
                self.display_todo(todo);
            }
        }

        if !found {
            println!("{}", "No matching tasks found!".yellow());
        }
        println!();
    }

    fn display_todo(&self, todo: &Todo) {
        let status = if todo.completed {
            "✓".green()
        } else {
            "○".yellow()
        };
        
        println!(
            "{} [{}] {} {} {}",
            status,
            todo.id.to_string().cyan(),
            todo.title.white(),
            todo.format_priority(),
            format!("(created: {})", 
                todo.created_at.format("%Y-%m-%d %H:%M")).dimmed()
        );

        if !todo.categories.is_empty() {
            println!(
                "     {} {}", 
                "↳ categories:".blue(),
                todo.categories.join(", ").dimmed()
            );
        }

        if let Some(due_date) = todo.due_date {
            println!(
                "     {} {}", 
                "↳ due:".yellow(),
                due_date.format("%Y-%m-%d").to_string().dimmed()
            );
        }

        if let Some(completed_at) = todo.completed_at {
            println!(
                "     {} {}", 
                "↳ completed:".green(),
                completed_at.format("%Y-%m-%d %H:%M").to_string().dimmed()
            );
        }
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
╭────────────────────────────────╮
│     RUST TODO MANAGER          │
╰────────────────────────────────╯"#.cyan());
}

fn main() -> io::Result<()> {
    print_banner();
    let mut todo_list = TodoList::new()?;
    let cli = Cli::from_args();

    match cli {
        Cli::Add { title, priority, due, tags } => {
            todo_list.add(title, priority, due, tags)?
        },
        Cli::List { completed, priority, tag } => {
            todo_list.list(completed, priority, tag)
        },
        Cli::Search { query } => todo_list.search(&query),
        Cli::Complete { id } => todo_list.complete(id)?,
        Cli::Delete { id } => todo_list.delete(id)?,
    }

    Ok(())
}