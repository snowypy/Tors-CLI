use std::fs;
use clap::{Command};
use colored::*;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: usize,
    name: String,
    description: String,
    eta: String,
    category: Option<String>,
}

fn setup_tors_directory() {
    let tors_dir = r"C:\Program Files\Tors\CLI";
    let config_file = format!("{}\\config.txt", tors_dir);
    let apikey_file = format!("{}\\apikey.txt", tors_dir);

    if !Path::new(tors_dir).exists() {
        fs::create_dir_all(tors_dir).expect("Unable to create Tors directory");
    }

    if !Path::new(&config_file).exists() {
        fs::write(&config_file, "http://localhost:3007").expect("Unable to create config file");
    }

    if !Path::new(&apikey_file).exists() {
        fs::write(&apikey_file, "DEMO_KEY").expect("Unable to create apikey file");
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Category {
    id: usize,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    theme: String,
}

#[tokio::main]
async fn main() {

    setup_tors_directory();

    let matches = Command::new("Tors CLI Tasks")
        .version("1.0")
        .author("Snowy - https://snowyjs.lol")
        .about("A multi-platform task manager.")
        .subcommand(Command::new("createtask").about("Creates a new task"))
        .subcommand(Command::new("createcategory").about("Creates a new category"))
        .subcommand(Command::new("edittask").about("Edits an existing task"))
        .subcommand(Command::new("editcategory").about("Edits an existing category"))
        .subcommand(Command::new("deltask").about("Deletes a task"))
        .subcommand(Command::new("delcategory").about("Deletes a category"))
        .subcommand(Command::new("assigncategory").about("Assigns a category to a task"))
        .subcommand(Command::new("listtasks").about("Lists all tasks with their categories"))
        .subcommand(Command::new("categories").about("Lists all categories"))
        .subcommand(Command::new("changetheme").about("Changes the theme"))
        .subcommand(Command::new("assignendpoint").about("Assigns the API endpoint"))
        .subcommand(Command::new("assignapikey").about("Assigns the API key"))
        .get_matches();

    let client = Client::new();
    let theme = get_theme(&client).await.unwrap_or_else(|_| "Default".to_string());
    let colors = get_colors(&theme);

    match matches.subcommand() {
        Some(("createtask", _)) => create_task(&client, &theme).await,
        Some(("createcategory", _)) => create_category(&client, &colors).await,
        Some(("edittask", _)) => edit_task(&client, &colors).await,
        Some(("editcategory", _)) => edit_category(&client, &colors).await,
        Some(("deltask", _)) => delete_task(&client, &colors).await,
        Some(("delcategory", _)) => delete_category(&client, &colors).await,
        Some(("assigncategory", _)) => assign_category(&client, &colors).await,
        Some(("listtasks", _)) => list_tasks(&client, &colors).await,
        Some(("categories", _)) => list_category(&client, &colors).await,
        Some(("changetheme", _)) => change_theme(&client, &colors).await,
        Some(("assignendpoint", _)) => assign_endpoint(&colors),
        Some(("assignapikey", _)) => assign_api_key(&colors),
        _ => println!("{}", "Invalid command. Use --help for usage.".color(hex_to_color(&colors.error))),
    }
}

async fn get_theme(client: &Client) -> Result<String, reqwest::Error> {
    let response = client
        .get(format!("{}/theme", get_api_url()))
        .header("api-key", format!(" {}", get_api_key()))
        .send()
        .await?;

    if response.status() == StatusCode::OK {
        let theme: Config = response.json().await?;
        Ok(theme.theme)
    } else {
        Ok("Default".to_string())
    }
}

struct Colors {
    success: String,
    error: String,
    warning: String,
    info: String,
}

fn get_colors(theme: &str) -> Colors {
    match theme {
        "Desert" => Colors {
            success: "#E5CAB7".to_string(),
            error: "#FF0000".to_string(),
            warning: "#FF00FF".to_string(),
            info: "#DCB8B0".to_string(),
        },
        "Oasis" => Colors {
            success: "#A6DFA7".to_string(),
            error: "#F7A1A3".to_string(),
            warning: "#F8C98B".to_string(),
            info: "#A5D8F3".to_string(),
        },
        "Forest" => Colors {
            success: "#B6D8B3".to_string(),
            error: "#E6A6A8".to_string(),
            warning: "#F1D8A5".to_string(),
            info: "#92C8C7".to_string(),
        },
        "Snow" => Colors {
            success: "#C8E3D4".to_string(),
            error: "#F4B6C2".to_string(),
            warning: "#F8E8A1".to_string(),
            info: "#B7E1F5".to_string(),
        },
        _ => Colors {
            success: "#E5CAB7".to_string(),
            error: "#FF0000".to_string(),
            warning: "#FF00FF".to_string(),
            info: "#DCB8B0".to_string(),
        },
    }
}

async fn fetch_theme(client: &Client) -> Result<String, reqwest::Error> {
    let response = client
        .get(format!("{}/theme", get_api_url()))
        .header("api-key", format!(" {}", get_api_key()))
        .send()
        .await?;

    if response.status() == StatusCode::OK {
        let theme: Config = response.json().await?;
        Ok(theme.theme)
    } else {
        Ok("Default".to_string())
    }
}

fn get_color_for_type(theme: &str, color_type: &str) -> String {
    let colors = get_colors(theme);
    match color_type {
        "success" => colors.success,
        "error" => colors.error,
        "warning" => colors.warning,
        "info" => colors.info,
        _ => "#000000".to_string(),
    }
}

fn hex_to_color(hex: &str) -> colored::Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    colored::Color::TrueColor { r, g, b }
}

fn prompt(message: &str) -> String {
    print!("{} ", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_api_url() -> String {
    fs::read_to_string(r"C:\Program Files\Tors\CLI\config.txt").expect("Unable to read config file")
}

fn get_api_key() -> String {
    fs::read_to_string(r"C:\Program Files\Tors\CLI\apikey.txt").expect("Unable to read config file")
}

fn assign_endpoint(colors: &Colors) {
    let api_url = prompt("Enter the API endpoint URL:");
    fs::write(r"C:\Program Files\Tors\config.txt", api_url).expect("Unable to write to config file");
    println!("{}", "API endpoint assigned successfully!".color(hex_to_color(&colors.success)));
}

fn assign_api_key(colors: &Colors) {
    let api_key = prompt("Enter the API key:");
    fs::write(r"C:\Program Files\Tors\apikey.txt", api_key).expect("Unable to write to config file");
    println!("{}", "API key assigned successfully!".color(hex_to_color(&colors.success)));
}

async fn create_task(client: &Client, theme: &str) {
    let name = prompt("Enter task name:");
    let description = prompt("Enter task description:");
    let eta = prompt("Enter task ETA:");

    let task = Task {
        id: 0, // [@] API Auto Increments ID
        name,
        description,
        eta,
        category: None,
    };

    let response = client
        .post(format!("{}/tasks", get_api_url()))
        .header("api-key", format!(" {}", get_api_key()))
        .json(&task)
        .send()
        .await;

    match response {
        Ok(resp) if resp.status() == StatusCode::CREATED => {
            let success_color = get_color_for_type(theme, "success");
            println!("{}", "Task created successfully!".color(hex_to_color(&success_color)));
        }
        Ok(resp) => {
            let error_color = get_color_for_type(theme, "error");
            println!("{}", format!("Failed to create task: {:?}", resp.text().await).color(hex_to_color(&error_color)));
        }
        Err(e) => {
            let error_color = get_color_for_type(theme, "error");
            println!("{}", format!("Error: {}", e).color(hex_to_color(&error_color)));
        }
    }
}

async fn create_category(client: &Client, colors: &Colors) {
    let name = prompt("Enter category name:");

    let category = Category {
        id: 0,
        name,
    };

    let response = client
        .post(format!("{}/categories", get_api_url()))
        .header("api-key", format!(" {}", get_api_key()))
        .json(&category)
        .send()
        .await;

    match response {
        Ok(resp) if resp.status() == StatusCode::CREATED => {
            println!("{}", "Category created successfully!".color(hex_to_color(&colors.success)));
        }
        Ok(resp) => {
            println!("{}", format!("Failed to create category: {:?}", resp.text().await).color(hex_to_color(&colors.error)));
        }
        Err(e) => {
            println!("{}", format!("Error: {}", e).color(hex_to_color(&colors.error)));
        }
    }
}

async fn list_tasks(client: &Client, colors: &Colors) {
    let response = client
        .get(format!("{}/tasks", get_api_url()))
        .header("api-key", format!(" {}", get_api_key()))
        .send()
        .await;

    match response {
        Ok(resp) if resp.status() == StatusCode::OK => {
            let tasks: Vec<Task> = resp.json().await.unwrap();
            if tasks.is_empty() {
                println!("{}", "No tasks found.".color(hex_to_color(&colors.warning)));
            } else {
                let mut tasks_by_category: std::collections::HashMap<String, Vec<Task>> = std::collections::HashMap::new();
                for task in tasks {
                    let category_name = match &task.category {
                        Some(category_id) => get_category_name(client, category_id).await.unwrap_or_else(|_| "Unknown".to_string()),
                        None => "No Category".to_string(),
                    };
                    tasks_by_category.entry(category_name).or_default().push(task);
                }

                for (category_name, tasks) in tasks_by_category {
                    println!("{}", format!("{}:", category_name).color(hex_to_color(&colors.info)));
                    for task in tasks {
                        println!(
                            "    {} [{}]:",
                            task.name.color(hex_to_color(&colors.success)),
                            task.id.to_string().bright_black()
                        );
                        println!("      - {}", task.description.white());
                        println!("      - ETA: {}", task.eta);
                    }
                }
            }
        }
        Ok(resp) => {
            println!("{}", format!("Failed to list tasks: {:?}", resp.text().await).color(hex_to_color(&colors.error)));
        }
        Err(e) => {
            println!("{}", format!("Error: {}", e).color(hex_to_color(&colors.error)));
        }
    }
}

async fn get_category_name(client: &Client, category_id: &str) -> Result<String, reqwest::Error> {
    let response = client
        .get(format!("{}/categories/{}", get_api_url(), category_id))
        .header("api-key", format!(" {}", get_api_key()))
        .send()
        .await?;

    if response.status() == StatusCode::OK {
        let category: Category = response.json().await?;
        Ok(category.name)
    } else {
        Ok("Unknown".to_string())
    }
}

async fn list_category(client: &Client, colors: &Colors) {
    let response = client
        .get(format!("{}/categories", get_api_url()))
        .header("api-key", format!(" {}", get_api_key()))
        .send()
        .await;

    match response {
        Ok(resp) if resp.status() == StatusCode::OK => {
            let categories: Vec<Category> = resp.json().await.unwrap();
            if categories.is_empty() {
                println!("{}", "No categories found.".color(hex_to_color(&colors.warning)));
            } else {
                for category in categories {
                    println!(
                        "{} {}{}{}",
                        category.name.color(hex_to_color(&colors.success)),
                        "[ID".bright_black(),
                        category.id.to_string().bright_black(),
                        "]".bright_black()
                    );
                }
            }
        }
        Ok(resp) => {
            println!("{}", format!("Failed to list categories: {:?}", resp.text().await).color(hex_to_color(&colors.error)));
        }
        Err(e) => {
            println!("{}", format!("Error: {}", e).color(hex_to_color(&colors.error)));
        }
    }
}

async fn change_theme(client: &Client, colors: &Colors) {
    let theme = prompt("Enter new theme (Desert, Oasis, Forest, Snow):");

    let valid_themes = vec!["Desert", "Oasis", "Forest", "Snow"];
    if !valid_themes.contains(&theme.as_str()) {
        println!("{}", "Invalid theme.".color(hex_to_color(&colors.error)));
        return;
    }

    let response = client
        .post(format!("{}/theme", get_api_url()))
        .header("api-key", format!(" {}", get_api_key()))
        .json(&serde_json::json!({ "newTheme": theme }))
        .send()
        .await;

    match response {
        Ok(resp) if resp.status() == StatusCode::OK => {
            println!("{}", "Theme changed successfully!".color(hex_to_color(&colors.success)));
        }
        Ok(resp) => {
            println!("{}", format!("Failed to change theme: {:?}", resp.text().await).color(hex_to_color(&colors.error)));
        }
        Err(e) => {
            println!("{}", format!("Error: {}", e).color(hex_to_color(&colors.error)));
        }
    }
}

async fn assign_category(client: &Client, colors: &Colors) {
    let task_id: usize = prompt("Enter task ID:").parse().unwrap();
    let category_id: usize = prompt("Enter category ID:").parse().unwrap();

    let response = client
        .post(format!("{}/tasks/{}/assign-category", get_api_url(), task_id))
        .header("api-key", format!(" {}", get_api_key()))
        .json(&serde_json::json!({ "categoryId": category_id }))
        .send()
        .await;

    match response {
        Ok(resp) if resp.status() == StatusCode::OK => {
            println!("{}", "Category assigned to task successfully!".color(hex_to_color(&colors.success)));
        }
        Ok(resp) => {
            println!("{}", format!("Failed to assign category: {:?}", resp.text().await).color(hex_to_color(&colors.error)));
        }
        Err(e) => {
            println!("{}", format!("Error: {}", e).color(hex_to_color(&colors.error)));
        }
    }
}

async fn delete_task(client: &Client, colors: &Colors) {
    let id: usize = prompt("Enter task ID to delete:").parse().unwrap();

    let response = client
        .delete(format!("{}/tasks/{}", get_api_url(), id))
        .header("api-key", format!(" {}", get_api_key()))
        .send()
        .await;

    match response {
        Ok(resp) if resp.status() == StatusCode::OK => {
            println!("{}", "Task deleted successfully!".color(hex_to_color(&colors.success)));
        }
        Ok(resp) => {
            println!("{}", format!("Failed to delete task: {:?}", resp.text().await).color(hex_to_color(&colors.error)));
        }
        Err(e) => {
            println!("{}", format!("Error: {}", e).color(hex_to_color(&colors.error)));
        }
    }
}

async fn delete_category(client: &Client, colors: &Colors) {
    let id: usize = prompt("Enter category ID to delete:").parse().unwrap();

    let response = client
        .delete(format!("{}/categories/{}", get_api_url(), id))
        .header("api-key", format!(" {}", get_api_key()))
        .send()
        .await;

    match response {
        Ok(resp) if resp.status() == StatusCode::OK => {
            println!("{}", "Category deleted successfully!".color(hex_to_color(&colors.success)));
        }
        Ok(resp) => {
            println!("{}", format!("Failed to delete category: {:?}", resp.text().await).color(hex_to_color(&colors.error)));
        }
        Err(e) => {
            println!("{}", format!("Error: {}", e).color(hex_to_color(&colors.error)));
        }
    }
}

async fn edit_task(client: &Client, colors: &Colors) {
    let task_id: usize = prompt("Enter task ID to edit:").parse().unwrap();
    let name = prompt("Enter new task name:");
    let description = prompt("Enter new task description:");
    let eta = prompt("Enter new task ETA:");

    let task = Task {
        id: task_id,
        name,
        description,
        eta,
        category: None,
    };

    let response = client
        .put(format!("{}/tasks/{}", get_api_url(), task_id))
        .header("api-key", format!(" {}", get_api_key()))
        .json(&task)
        .send()
        .await;

    match response {
        Ok(resp) if resp.status() == StatusCode::OK => {
            println!("{}", "Task updated successfully!".color(hex_to_color(&colors.success)));
        }
        Ok(resp) => {
            println!("{}", format!("Failed to update task: {:?}", resp.text().await).color(hex_to_color(&colors.error)));
        }
        Err(e) => {
            println!("{}", format!("Error: {}", e).color(hex_to_color(&colors.error)));
        }
    }
}

async fn edit_category(client: &Client, colors: &Colors) {
    let category_id: usize = prompt("Enter category ID to edit:").parse().unwrap();
    let name = prompt("Enter new category name:");

    let category = Category {
        id: category_id,
        name,
    };

    let response = client
        .put(format!("{}/categories/{}", get_api_url(), category_id))
        .header("api-key", format!(" {}", get_api_key()))
        .json(&category)
        .send()
        .await;

    match response {
        Ok(resp) if resp.status() == StatusCode::OK => {
            println!("{}", "Category updated successfully!".color(hex_to_color(&colors.success)));
        }
        Ok(resp) => {
            println!("{}", format!("Failed to update category: {:?}", resp.text().await).color(hex_to_color(&colors.error)));
        }
        Err(e) => {
            println!("{}", format!("Error: {}", e).color(hex_to_color(&colors.error)));
        }
    }
}