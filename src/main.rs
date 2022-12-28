//! # rtodo
//!
//! I wrote this project while I'm learning Rust for practice.
//! I decided to put this project online so that others, like me,
//! who are learning Rust can take some ideas and help me improve this code too.
//!
//! Feel free to open a pull request to improve the code or to
//! add new features and help me learn Rust.

use colored::Colorize;

use std::process::Command;

use rtodo::todos::Todo;
use rtodo::{add_todo, main_menu, remove_todo, show_todos, sub_menu, update_todo, user_input};

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
            std::process::exit(0);
        }
    } // End main loop
}
