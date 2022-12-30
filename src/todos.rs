use std::fmt;

use serde_derive::{Deserialize, Serialize};

/// Enum to keep the priority of todos
#[derive(Serialize, Deserialize)]
pub enum Priority {
    HIGH,
    MEDIUM,
    LOW,
}

/// Implement the Display trait for the enum Priority
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Priority::LOW => write!(f, "LOW"),
            Priority::MEDIUM => write!(f, "MEDIUM"),
            Priority::HIGH => write!(f, "HIGH"),
        }
    }
}

/// Struct to keep the todos
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub done: bool,
    pub time: Priority,
    pub date: String,
}

impl Todo {
    // Constructor for initializing the Todo
    pub fn new(
        title: String,
        description: String,
        done: bool,
        time: Priority,
        date: String,
    ) -> Self {
        Self {
            title,
            description,
            done,
            time,
            date,
        }
    }
}
