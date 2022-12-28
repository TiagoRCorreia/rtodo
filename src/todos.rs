use serde_derive::{Deserialize, Serialize};
/// Struct to keep the todos
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub description: String,
}

impl Todo {
    // Constructor for initializing the Todo
    pub fn new(title: String, description: String) -> Self {
        Self { title, description }
    }
}
