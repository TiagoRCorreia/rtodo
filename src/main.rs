//! # rtodo
//!
//! I wrote this project while I'm learning Rust for practice.
//! I decided to put this project online so that others, like me,
//! who are learning Rust can take some ideas and help me improve this code too.
//!
//! Feel free to open a pull request to improve the code or to
//! add new features and help me learn Rust.

// Read input and flushing
use std::io::{self, Write};

// For Command and Exit
use std::process::{self, Command};

// For colored output
use colored::Colorize;

use todos::Todo;

pub mod todos;

/// Get user input and return the value as a String
fn user_input() -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

/// Add todos into `Vec<Todo>`
fn add_todo(td: &mut Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    print!("{}{}", "Title ".white().bold(), "-> ".green().bold());
    let title = user_input()?.trim().to_string();

    print!("{}{}", "Description".white().bold(), "-> ".green().bold());
    let desc = user_input()?.trim().to_string();

    td.push(Todo::new(title, desc));
    Ok(())
}

/// Show todos in a formatted table
fn show_todos(todos: &Vec<Todo>) {
    // Clear the screen
    Command::new("clear").status().unwrap();

    println!(
        "{:^10} {:^40} {:^40}",
        "ID".blue().bold(),
        "Title".blue().bold(),
        "Description".blue().bold()
    );

    // Format the output with hyphens
    println!(
        "{:-^10} {:-^40} {:-^40}",
        "".blue().bold(),
        "".blue().bold(),
        "".blue().bold()
    );

    // Display todos
    if !todos.is_empty() {
        for (i, x) in todos.iter().enumerate() {
            println!(
                "{:^10} {:^40} {:^40}",
                i.to_string().blue().bold(),
                x.title.cyan().bold(),
                x.description.yellow().bold()
            );
        }
    }
}

/// Removes todo with the given ID from the list
fn remove_todo(todos: &mut Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    print!(
        "\n{} {}{}",
        "ID".red().bold(),
        "to remove ".white().bold(),
        "-> ".green().bold()
    );

    // Get ID from user and parse it into usize
    let rm = user_input()?.trim().to_string().parse::<usize>()?;

    // Check if `Vec<Todo>` is not empty and remove the Todo
    if rm < todos.len() && !todos.is_empty() {
        todos.remove(rm);
    };

    Ok(())
}

/// Update todo with the given ID
fn update_todo(todos: &mut [Todo]) -> Result<(), Box<dyn std::error::Error>> {
    print!(
        "\n{} {}{}",
        "ID".red().bold(),
        " to update ".white().bold(),
        "-> ".green().bold()
    );

    // Get ID from user
    let id = user_input()?.trim().to_string();
    print!("{}{}", "Title ".white().bold(), "-> ".green().bold());

    // Get Title from user
    let title = user_input()?.trim().to_string();
    print!("{}{}", "Description ".white().bold(), "-> ".white().bold());

    // Get Desc from user
    let desc = user_input()?.trim().to_string();

    // Parse ID string into a usize
    let id = id.trim().to_string().parse::<usize>()?;

    // Find the ID and update it with the given values
    for (i, z) in todos.iter_mut().enumerate() {
        if i == id {
            z.title = title.to_string();
            z.description = desc.to_string();
        }
    }
    Ok(())
}

/// Display the main menu
fn main_menu() {
    println!(
        "\n{:>5} {}\n{:>5} {}\n{:>5} {}\n{:>5} {}\n{:>5} {}\n",
        "[1]".blue(),
        "Show Todos".yellow(),
        "[2]".blue(),
        "Add Todos".yellow(),
        "[3]".blue(),
        "Update Todos".yellow(),
        "[4]".blue(),
        "Remove Todos".yellow(),
        "[5]".blue(),
        "Exit".yellow(),
    );
}

/// Display the sub menu
fn sub_menu(todos: &mut Vec<Todo>) -> Result<bool, Box<dyn std::error::Error>> {
    print!(
        "\n\n{} {:<10} {} {:<10} {} {:<10}",
        "[1]".blue().bold(),
        "Main Menu".white().bold(),
        "[2]".blue().bold(),
        "Add Todo".white().bold(),
        "[3]".blue().bold(),
        "Remove Todo -> ".white().bold(),
    );

    let id = user_input()?.trim().to_string();

    if id.contains('1') {
        show_todos(todos);
        Ok(false)
    } else if id.contains('2') {
        add_todo(todos)?;
        Ok(true)
    } else if id.contains('3') {
        remove_todo(todos)?;
        Ok(true)
    } else {
        println!("Option Invalid!");
        Ok(false)
    }
}

fn main() {
    // Create a vector to hold the todos
    let mut todos: Vec<Todo> = vec![];

    // Main loop
    loop {
        // Clear the screen
        Command::new("clear").status().unwrap();

        // Show main menu
        main_menu();

        // Ask user to choose an option
        print!(
            "{}{}",
            "Choose an option ".white().bold(),
            "-> ".green().bold()
        );

        // Check if input from user is valid
        let user = match user_input() {
            Ok(u) => u.trim().to_string(),
            Err(e) => panic!("Error get user input {e}"),
        };

        // Open sub menu / show todos
        if user.contains('1') {
            show_todos(&todos);

            // Display the sub menu in a loop
            while let Ok(e) = sub_menu(&mut todos) {
                // Break if `sub_menu()` returned false
                if !e {
                    break;
                }
                show_todos(&todos);
            }
        // add todo
        } else if user.contains('2') {
            if let Err(e) = add_todo(&mut todos) {
                println!("Error add todo!!! {e}");
            }
        // update todo
        } else if user.contains('3') {
            show_todos(&todos);
            if let Err(e) = update_todo(&mut todos) {
                println!("Error update todo!!! {e}");
            }
        // remove todo
        } else if user.contains('4') {
            show_todos(&todos);
            if let Err(e) = remove_todo(&mut todos) {
                println!("Error remove todo!!! {e}");
            }
        // exit
        } else if user.contains('5') {
            process::exit(0);
        }
    } // End main loop
}
