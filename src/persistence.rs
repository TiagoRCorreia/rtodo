use std::fs;

use anyhow::Result;
use csv::{Reader, Writer};
use directories::BaseDirs;

use colored::Colorize;

use crate::{errors::TErrors, todos::Todo, user_input};

// Read todos from a file
pub fn read_from_file() -> Result<Vec<Todo>> {
    // Get todos file
    let path = BaseDirs::new().unwrap().config_dir().join("rtodo/db.json");
    // Read todos from the file as a string
    let td_str = std::fs::read_to_string(path)?;

    // Create a vector of todos
    let todos: Vec<Todo> = serde_json::from_str(&td_str).map_err(|_| TErrors::ReadFromFile)?;

    // return todos
    Ok(todos)
}

// Write todos into a file
pub fn write_to_file(todos: &Vec<Todo>) -> Result<()> {
    // Get the .config directory on the Linux system
    let path = BaseDirs::new().unwrap().config_dir().join("rtodo");

    // Create rtodo directory
    if !path.exists() {
        std::fs::create_dir(&path)?;
    }

    // Write todos into the file
    std::fs::write(path.join("db.json"), serde_json::to_string(&todos)?)
        .map_err(|_| TErrors::WriteToFile)?;
    Ok(())
}

/// Import menu
pub fn import_menu(td: &mut Vec<Todo>) -> Result<()> {
    print!("{}", "Format to import: ".magenta());
    print!(
        "\n{}{} {}{} {}",
        "[1]".blue().bold(),
        "CSV".white().bold(),
        "[2]".blue().bold(),
        "JSON".white().bold(),
        " -> ".green().bold()
    );

    let format = user_input()
        .map_err(|e| TErrors::ImportMenu(e.to_string()))?
        .trim()
        .to_string();

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
pub fn export_menu(td: &Vec<Todo>) -> Result<()> {
    print!("{}", "Format to export: ".magenta());
    print!(
        "\n{}{} {}{} {}",
        "[1]".blue().bold(),
        "CSV".white().bold(),
        "[2]".blue().bold(),
        "JSON".white().bold(),
        " -> ".green().bold()
    );

    let format = user_input()
        .map_err(|e| TErrors::ExportMenu(e.to_string()))?
        .trim()
        .to_string();

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
pub fn export_to_csv(td: &[Todo], name: &str) -> Result<()> {
    // Prepare CSV file
    let mut cs_writer = Writer::from_path(name).map_err(|_| TErrors::WriteToFile)?;

    // Write into the file
    for t in td {
        cs_writer.serialize(t)?;
        cs_writer.flush()?;
    }

    Ok(())
}

/// Export todos into the JSON file
fn export_to_json(td: &Vec<Todo>, name: &str) -> Result<()> {
    // Write data to the file
    std::fs::write(name, serde_json::to_string_pretty(td)?).map_err(|_| TErrors::WriteToFile)?;

    Ok(())
}

/// Import todos from the JSON file
fn import_from_json(td: &mut Vec<Todo>, name: &str) -> Result<()> {
    // Read contents of file
    let dt = fs::read_to_string(name).map_err(|_| TErrors::ReadFromFile)?;

    // Deserialize data into the vector
    let rs: Vec<Todo> = serde_json::from_str(&dt)?;

    // Append into the todos vector
    td.extend(rs);

    Ok(())
}

/// Import from CSV file
pub fn import_from_csv(td: &mut Vec<Todo>, name: &str) -> Result<()> {
    // Reaad CSV file
    let mut read_csv = Reader::from_path(name).map_err(|_| TErrors::WriteToFile)?;

    // Get an iterator
    let it = read_csv.deserialize::<Vec<Todo>>();

    // Append into the vector todos
    for t in it {
        td.extend(t.unwrap());
    }

    Ok(())
}
