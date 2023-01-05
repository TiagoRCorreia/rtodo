use std::cmp::Ordering;

use anyhow::Result;
use colored::Colorize;

use crate::{
    errors::TErrors,
    todos::{Priority, Todo},
    user_input,
};

/// Menu sort
pub fn sorting_menu(td: &mut [Todo]) -> Result<()> {
    // Display menu sort
    print!("\n{}", "Sort by:".magenta());
    print!(
        "\n\n{}{:^5} {}{:^5} {}",
        "[1]".blue().bold(),
        "Status".white().bold(),
        "[2]".blue().bold(),
        "Priority".white().bold(),
        " -> ".green().bold(),
    );

    let sort = user_input()
        .map_err(|e| TErrors::SortMenu(e.to_string()))?
        .trim()
        .to_string();

    // Sort by status
    if sort.contains('1') {
        // Display menu
        print!("\n{}", "Sort by status:".magenta());
        print!(
            "\n\n{}{} {}{} {}",
            "[1]".blue().bold(),
            "Done".green().bold(),
            "[2]".blue().bold(),
            "Undone".red().bold(),
            "-> ".green().bold(),
        );

        // Get user input
        let sort_status = user_input()?.trim().to_string();

        if sort_status.contains('1') {
            // Sort by done
            sort_by_status(td, true);
        } else if sort_status.contains('2') {
            // Sort by undone
            sort_by_status(td, false);
        }

        // Sort by priority
    } else if sort.contains('2') {
        print!("\n{}", "Sort by priority:".magenta());
        print!(
            "\n\n{}{} {}{} {}{} {}",
            "[1]".blue().bold(),
            "LOW".green().bold(),
            "[2]".blue().bold(),
            "MEDIUM".yellow().bold(),
            "[3]".blue().bold(),
            "HIGH".red().bold(),
            "-> ".green().bold(),
        );

        let sort_priority = user_input()?.trim().to_string();

        if sort_priority.contains('1') {
            // Sort priority by low
            sort_by_priority(td, Priority::LOW);
        } else if sort_priority.contains('2') {
            // Sort priority by Medium
            sort_by_priority(td, Priority::MEDIUM);
        } else if sort_priority.contains('3') {
            // Sort priority by High
            sort_by_priority(td, Priority::HIGH);
        }
    }

    Ok(())
}

/// Sorting by priority
fn sort_by_priority(td: &mut [Todo], pr: Priority) {
    match pr {
        Priority::LOW => {
            td.sort_by(|x, _| {
                if x.time.to_string().contains("LOW") {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
        }
        Priority::MEDIUM => {
            td.sort_by(|x, _| {
                if x.time.to_string().contains("MEDIUM") {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
        }
        Priority::HIGH => {
            td.sort_by(|x, _| {
                if x.time.to_string().contains("HIGH") {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
        }
    }
}

/// Sorting by status
fn sort_by_status(td: &mut [Todo], switch: bool) {
    td.sort_by(|x, _| {
        // if true sort by done
        if x.done == switch {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}
