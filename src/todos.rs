use serde_derive::{Deserialize, Serialize};
/// Struct to keep the todos
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub done: bool,
}

impl Todo {
    // Constructor for initializing the Todo
    pub fn new(title: String, description: String, done: bool) -> Self {
        Self {title, description, done, }
    }
}
