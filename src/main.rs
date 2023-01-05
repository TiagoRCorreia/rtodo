//! # rtodo
//!
//! I wrote this project while I'm learning Rust for practice.
//! I decided to put this project online so that others, like me,
//! who are learning Rust can take some ideas and help me improve this code too.
//!
//! Feel free to open a pull request to improve the code or to
//! add new features and help me learn Rust.

use colored::Colorize;
use rtodo::commands::execute_commands;

use std::process::Command;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use rtodo::persistence::{self, export_menu, import_menu};
use rtodo::todos::Todo;
use rtodo::{
    add_todo, main_menu, remove_todo, save_and_exit, show_todos, sub_menu, update_todo, user_input,
};

fn main() {
    // Create an atomic variable wrapped in an Arc pointer
    let running = Arc::new(AtomicBool::new(true));

    // Clone pointer for later use
    let first_catch = running.clone();
    let last_catch = running.clone();

    // Create a handler to catch CTRL+C
    ctrlc::set_handler(move || {
        first_catch.store(false, Ordering::SeqCst);
    })
    .expect("Error creating a handler!!!");

    // Create a vector to hold the todos
    let mut todos: Vec<Todo> = vec![];

    // Try reading todos from the file
    if let Ok(vec) = persistence::read_from_file() {
        todos.extend(vec);
    }

    // Get command and execute
    if let Err(e) = execute_commands(&mut todos) {
        eprintln!("{e}");
    }

    // Main loop
    loop {
        // Handler loop
        while running.load(Ordering::SeqCst) {
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
                Err(e) => panic!("{e}"),
            };

            // Open sub menu / show todos
            if user.contains('1') {
                show_todos(&mut todos);

                // Display the sub menu in a loop
                while let Ok(e) = sub_menu(&mut todos) {
                    // Break if `sub_menu()` returned false
                    if !e {
                        break;
                    }
                    show_todos(&mut todos);
                }
            // add todo
            } else if user.contains('2') {
                if let Err(e) = add_todo(&mut todos) {
                    eprintln!("{e}");
                }
            // update todo
            } else if user.contains('3') {
                show_todos(&mut todos);
                if let Err(e) = update_todo(&mut todos) {
                    eprintln!("{e}");
                }
            // remove todo
            } else if user.contains('4') {
                show_todos(&mut todos);
                if let Err(e) = remove_todo(&mut todos) {
                    eprintln!("{e}");
                }
                // Import
            } else if user.contains('5') {
                if let Err(e) = import_menu(&mut todos) {
                    eprintln!("{e}");
                }
                // Export
            } else if user.contains('6') {
                if let Err(e) = export_menu(&todos) {
                    eprintln!("{e}");
                }
            // exit
            } else if user.contains('7') {
                if let Err(e) = persistence::write_to_file(&todos) {
                    eprintln!("{e}");
                }
                std::process::exit(0);
            }
        } // End while loop

        // Display message
        print!(
            "{}{}{}",
            "Do you want exit?".red().bold(),
            " [Y/N]".yellow().bold(),
            " -> ".green().bold()
        );

        // Get confirmation from the user
        if let Ok(confirm) = user_input() {
            // Check if the user wants to exit.
            if confirm.trim().eq_ignore_ascii_case("y") {
                // Save and exit
                save_and_exit(&mut todos);
            } else {
                // Write true into the handler
                last_catch.store(true, Ordering::SeqCst);
            }
        }
    } // End main loop
}
