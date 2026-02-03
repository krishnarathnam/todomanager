use chrono;
use colorize::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::from_str;
use std::{fs, io::Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub created_at: String,
    pub title: String,
    pub done: bool,
    pub id: u32,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub data: Vec<Todo>,
}

pub struct Command {
    pub command: String,
    pub arguments: Vec<String>,
}

pub fn init() {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .expect("Cannot find home directory");

    let mut app_dir = std::path::PathBuf::from(home_dir);
    app_dir.push(".todomanager");

    let mut data_file = app_dir.clone();
    data_file.push("todo.json");

    if !app_dir.exists() {
        fs::create_dir(&app_dir).unwrap();
    }

    if !data_file.exists() {
        let mut file = fs::File::create(&data_file).unwrap();
        file.write_all(b"{\"data\": []}").unwrap();
        println!("Created central todo storage at {:?}", data_file);
    }
}

pub fn data_path() -> std::path::PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .expect("Cannot find home dir");

    let mut path = std::path::PathBuf::from(home);
    path.push(".todomanager/todo.json");
    path
}

pub fn get_args() -> Command {
    let args: Vec<String> = std::env::args().collect();

    let command = args.get(1).unwrap_or(&"".to_string()).to_string();
    let arguments: Vec<String> = args.iter().skip(2).cloned().collect();

    Command { command, arguments }
}

pub fn get_timestamp() -> String {
    let now = chrono::Local::now();
    let timestamp = now.format("%m-%d %H:%M").to_string();

    timestamp
}

pub fn get_id() -> u32 {
    let mut rng = rand::rng();
    let id: u32 = rng.random_range(1..100);

    id + rng.random_range(1..100)
}

pub fn get_todo() -> Result<Vec<Todo>> {
    let path = data_path();
    let data = fs::read_to_string(path).unwrap();
    let todos: ConfigFile = from_str(&data)?;

    Ok(todos.data)
}

pub fn save_todo(todos: Vec<Todo>) {
    let config_file = ConfigFile { data: todos };
    let json = serde_json::to_string(&config_file).unwrap();

    let mut file = fs::File::create(data_path()).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
pub fn help() {
    let help = format!(
        "
    Command   |  Description
    {}          Add a new todo
    {}         List all todos
    {}       Mark and unmark a todo as done
    {}       Delete a todo
    {}         Sort todos (pending first, completed last)
    {}        Delete all todos
    ",
        "add".cyan(),
        "list".blue(),
        "toggle".green(),
        "remove".red(),
        "sort".yellow(),
        "reset".magenta()
    );

    println!("{help}");
}

pub fn add(titles: Vec<String>) {
    if titles.is_empty() {
        println!("{}", "No title provided".red());
        return;
    }

    let mut todos = get_todo().unwrap();

    for title in titles {
        if title.trim().is_empty() {
            continue;
        }
        
        let todo = Todo {
            created_at: get_timestamp(),
            title: title.trim().to_string(),
            done: false,
            id: get_id(),
            updated_at: get_timestamp(),
        };

        todos.push(todo);
    }

    save_todo(todos);
    list();
}

pub fn reset() {
    let mut file = fs::File::create(data_path()).unwrap();
    file.write_all(b"{\"data\": []}").unwrap();
    println!("{}", "Deleted all task".red());
}

pub fn print_list(todos: Vec<Todo>) {
    let max_title_width = todos.iter().map(|t| t.title.len()).max().unwrap_or(0);
    let max_id_width = todos
        .iter()
        .map(|t| t.id.to_string().len())
        .max()
        .unwrap_or(0);

    for todo in todos {
        println!(
            "{} {:>id_w$} {:<title_w$} @ {}",
            if todo.done {
                "✓".green()
            } else {
                "✗".red()
            },
            todo.id,
            todo.title,
            todo.created_at,
            id_w = max_id_width,
            title_w = max_title_width,
        );
    }
}

pub fn sort() {
    let mut todos = get_todo().unwrap();
    todos.sort_by_key(|todo| todo.done);

    print_list(todos);
}

pub fn list() {
    let todos = get_todo().unwrap();

    if todos.len() == 0 {
        println!("{}", "No Todo".red());
        return;
    }

    print_list(todos);
}

pub fn toggle(id: String) {
    let mut todos = get_todo().unwrap();
    let id = id.trim().parse::<u32>().unwrap_or(0);

    let exists = todos.iter().any(|todo| todo.id == id);

    if !exists {
        println!("{}", "todo not found".red());
        return;
    }

    for todo in &mut todos {
        if todo.id == id {
            todo.done = !todo.done;
            todo.updated_at = get_timestamp();
        }
    }

    save_todo(todos);
    list();
}
pub fn remove(id: String) {
    let mut todos = get_todo().unwrap();
    let id = id.trim().parse::<u32>().unwrap_or(0);

    let exists = todos.iter().any(|todo| todo.id == id);

    if !exists {
        println!("{}", "todo not found".red());
        return;
    }

    todos.retain(|todo| todo.id != id);
    save_todo(todos);

    list();
}
