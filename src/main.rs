use clap::{Command};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: usize,
    name: String,
    description: String,
    eta: String,
    category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Category {
    id: usize,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    tasks: Vec<Task>,
    categories: Vec<Category>,
    theme: String,
}

impl Config {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            categories: Vec::new(),
            theme: "Desert".to_string(),
        }
    }

    fn save(&self, path: &str) {
        let yaml = serde_yaml::to_string(self).unwrap();
        fs::write(path, yaml).expect("Unable to write to file");
    }

    fn load(path: &str) -> Self {
        if Path::new(path).exists() {
            let mut file = File::open(path).expect("Unable to open file");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("Unable to read file");
            serde_yaml::from_str(&content).unwrap()
        } else {
            Self::new()
        }
    }
}

fn main() {
    let config_path = "task_manager.yaml";
    let mut config = Config::load(config_path);

    let matches = Command::new("Tors CLI Tasks")
        .version("1.0")
        .author("Snowy - https://snowyjs.lol")
        .about("A multi platform task manager.")
        .subcommand(Command::new("createtask").about("Creates a new task"))
        .subcommand(Command::new("createcategory").about("Creates a new category"))
        .subcommand(Command::new("edittask").about("Edits an existing task"))
        .subcommand(Command::new("editcategory").about("Edits an existing category"))
        .subcommand(Command::new("deltask").about("Deletes a task"))
        .subcommand(Command::new("delcategory").about("Deletes a category"))
        .subcommand(Command::new("assigncategory").about("Assigns a category to a task"))
        .subcommand(Command::new("listtasks").about("Lists all tasks with their categories"))
        .subcommand(Command::new("changetheme").about("Changes the theme"))
        .get_matches();

    match matches.subcommand() {
        Some(("createtask", _)) => create_task(&mut config),
        Some(("createcategory", _)) => create_category(&mut config),
        Some(("edittask", _)) => edit_task(&mut config),
        Some(("editcategory", _)) => edit_category(&mut config),
        Some(("deltask", _)) => delete_task(&mut config),
        Some(("delcategory", _)) => delete_category(&mut config),
        Some(("assigncategory", _)) => assign_category(&mut config),
        Some(("listtasks", _)) => list_tasks(&config),
        Some(("changetheme", _)) => change_theme(&mut config),
        _ => println!("{}", "Invalid command. Use --help for usage.".red()),
    }

    config.save(config_path);
}

fn prompt(message: &str) -> String {
    print!("{} ", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn apply_theme(theme: &str) -> fn(&str) -> ColoredString {
    match theme {
        "Desert" => |text| text.yellow(),
        "Oasis" => |text| text.cyan(),
        "Forest" => |text| text.green(),
        "Snow" => |text| text.white(),
        _ => |text| text.blue(),
    }
}

fn create_task(config: &mut Config) {
    let name = prompt("Enter task name:");
    let description = prompt("Enter task description:");
    let eta = prompt("Enter task ETA:");

    let id = config.tasks.len() + 1;
    config.tasks.push(Task {
        id,
        name,
        description,
        eta,
        category: None,
    });

    println!("{}", "Task created successfully!".green());
}

fn create_category(config: &mut Config) {
    let name = prompt("Enter category name:");

    let id = config.categories.len() + 1;
    config.categories.push(Category { id, name });

    println!("{}", "Category created successfully!".green());
}

fn edit_task(config: &mut Config) {
    let id: usize = prompt("Enter task ID to edit:").parse().unwrap();
    let field = prompt("Enter field to edit (name, description, eta):");
    let value = prompt("Enter new value:");

    if let Some(task) = config.tasks.iter_mut().find(|t| t.id == id) {
        match field.as_str() {
            "name" => task.name = value,
            "description" => task.description = value,
            "eta" => task.eta = value,
            _ => println!("{}", "Invalid field.".red()),
        }
        println!("{}", "Task updated successfully!".green());
    } else {
        println!("{}", "Task not found.".red());
    }
}

fn edit_category(config: &mut Config) {
    let id: usize = prompt("Enter category ID to edit:").parse().unwrap();
    let name = prompt("Enter new name:");

    if let Some(category) = config.categories.iter_mut().find(|c| c.id == id) {
        category.name = name;
        println!("{}", "Category updated successfully!".green());
    } else {
        println!("{}", "Category not found.".red());
    }
}

fn delete_task(config: &mut Config) {
    let id: usize = prompt("Enter task ID to delete:").parse().unwrap();

    if config.tasks.iter().any(|t| t.id == id) {
        config.tasks.retain(|t| t.id != id);
        println!("{}", "Task deleted successfully!".green());
    } else {
        println!("{}", "Task not found.".red());
    }
}

fn delete_category(config: &mut Config) {
    let id: usize = prompt("Enter category ID to delete:").parse().unwrap();

    if config.categories.iter().any(|c| c.id == id) {
        config.categories.retain(|c| c.id != id);
        println!("{}", "Category deleted successfully!".green());
    } else {
        println!("{}", "Category not found.".red());
    }
}

fn assign_category(config: &mut Config) {
    let task_id: usize = prompt("Enter task ID:").parse().unwrap();
    let category_id: usize = prompt("Enter category ID:").parse().unwrap();

    let category_name = config
        .categories
        .iter()
        .find(|c| c.id == category_id)
        .map(|c| c.name.clone());

    if let Some(category_name) = category_name {
        if let Some(task) = config.tasks.iter_mut().find(|t| t.id == task_id) {
            task.category = Some(category_name.clone());
            println!(
                "{}",
                format!("Task {} assigned to category {}.", task_id, category_name).green()
            );
        } else {
            println!("{}", "Task not found.".red());
        }
    } else {
        println!("{}", "Category not found.".red());
    }
}

fn list_tasks(config: &Config) {
    if config.tasks.is_empty() {
        println!("{}", "No tasks found.".yellow());
        return;
    }

    let mut category_map: std::collections::HashMap<String, Vec<&Task>> = std::collections::HashMap::new();
    for task in &config.tasks {
        let category_name = task.category.clone().unwrap_or_else(|| "Uncategorized".to_string());
        category_map
            .entry(category_name)
            .or_insert_with(Vec::new)
            .push(task);
    }

    let theme_color = apply_theme(&config.theme);

    for (category, tasks) in category_map.iter() {
        println!("{}", theme_color(category));

        for task in tasks {
            println!(
                "    {} {}{}{}",
                theme_color(&task.name),
                "[ID".bright_black(),
                task.id.to_string().bright_black(),
                "]".bright_black()
            );
            println!("        - {}", task.description.white());
            println!("        - {}", theme_color(&task.eta));
        }
    }
}


fn change_theme(config: &mut Config) {
    let theme = prompt("Enter new theme (Desert, Oasis, Forest, Snow):");

    match theme.as_str() {
        "Desert" | "Oasis" | "Forest" | "Snow" => {
            config.theme = theme.to_string();
            println!("{}", apply_theme(&config.theme)("Theme changed successfully!"));
        }
        _ => println!("{}", "Invalid theme.".red()),
    }
}
