use std::fs;

use csv::{Reader, Writer};
use directories::BaseDirs;

use colored::Colorize;

use crate::{todos::Todo, user_input};

// Read todos from a file
pub fn read_from_file() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    // Get todos file
    let path = BaseDirs::new().unwrap().config_dir().join("rtodo/db.json");
    // Read todos from the file as a string
    let td_str = std::fs::read_to_string(path)?;

    // Create a vector of todos
    let todos: Vec<Todo> = serde_json::from_str(&td_str)?;

    // return todos
    Ok(todos)
}

// Write todos into a file
pub fn write_to_file(todos: &Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    // Get the .config directory on the Linux system
    let path = BaseDirs::new().unwrap().config_dir().join("rtodo");

    // Create rtodo directory
    if !path.exists() {
        std::fs::create_dir(&path)?;
    }

    // Write todos into the file
    std::fs::write(path.join("db.json"), serde_json::to_string(&todos)?)?;
    Ok(())
}

/// Import menu
pub fn import_menu(td: &mut Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    print!("{}", "Format to import: ".magenta());
    print!(
        "\n{}{} {}{} {}",
        "[1]".blue().bold(),
        "CSV".white().bold(),
        "[2]".blue().bold(),
        "JSON".white().bold(),
        " -> ".green().bold()
    );

    let format = user_input()?.trim().to_string();

    if format.contains('1') {
        print!("{} {}", "File Name".white().bold(), " -> ".green().bold());
        let name = user_input()?.trim().to_string();
        import_from_csv(td, &name)?;
    } else if format.contains('2') {
        print!("{} {}", "File Name".white().bold(), " -> ".green().bold());
        let name = user_input()?.trim().to_string();
        import_from_json(td, &name)?;
    }

    Ok(())
}

/// Export menu
pub fn export_menu(td: &Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    print!("{}", "Format to export: ".magenta());
    print!(
        "\n{}{} {}{} {}",
        "[1]".blue().bold(),
        "CSV".white().bold(),
        "[2]".blue().bold(),
        "JSON".white().bold(),
        " -> ".green().bold()
    );

    let format = user_input()?.trim().to_string();

    if format.contains('1') {
        print!("{} {}", "File Name".white().bold(), " -> ".green().bold());
        let name = user_input()?.trim().to_string();
        export_to_csv(td, &name)?;
    } else if format.contains('2') {
        print!("{} {}", "File Name".white().bold(), " -> ".green().bold());
        let name = user_input()?.trim().to_string();
        export_to_json(td, &name)?;
    }

    Ok(())
}

/// Export todos into CSV file
pub fn export_to_csv(td: &[Todo], name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Prepare CSV file
    let mut cs_writer = Writer::from_path(name)?;

    // Write into the file
    for t in td {
        cs_writer.serialize(t)?;
        cs_writer.flush()?;
    }

    Ok(())
}

/// Export todos into the JSON file
fn export_to_json(td: &Vec<Todo>, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Write data to the file
    std::fs::write(name, serde_json::to_string_pretty(td)?)?;

    Ok(())
}

/// Import todos from the JSON file
fn import_from_json(td: &mut Vec<Todo>, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read contents of file
    let dt = fs::read_to_string(name)?;

    // Deserialize data into the vector
    let rs: Vec<Todo> = serde_json::from_str(&dt)?;

    // Append into the todos vector
    td.extend(rs);

    Ok(())
}

/// Import from CSV file
pub fn import_from_csv(td: &mut Vec<Todo>, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Reaad CSV file
    let mut read_csv = Reader::from_path(name)?;

    // Get an iterator
    let it = read_csv.deserialize::<Vec<Todo>>();

    // Append into the vector todos
    for t in it {
        td.extend(t.unwrap());
    }

    Ok(())
}
