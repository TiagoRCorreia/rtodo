use thiserror::Error;

#[derive(Error, Debug)]
pub enum TErrors {
    #[error("Error read user input")]
    ReadUserInput(#[from] std::io::Error),
    #[error("Error writing to file")]
    WriteToFile,
    #[error("Error reading from file")]
    ReadFromFile,
    #[error("Error parse ID!")]
    ParseID,
    #[error("Error add todo! {0}")]
    AddTodo(String),
    #[error("Error display submenu! {0}")]
    SubMenu(String),
    #[error("Error remove todo! {0}")]
    RemoveTodo(String),
    #[error("Error update todo! {0}")]
    UpdateTodo(String),
    #[error("Error SetDone todo! {0}")]
    SetDone(String),
    #[error("Error SetUnDone todo! {0}")]
    SetUnDone(String),
    #[error("Error set priority! {0}")]
    SetPriority(String),
    #[error("Error display export menu! {0}")]
    ExportMenu(String),
    #[error("Error display import menu! {0}")]
    ImportMenu(String),
    #[error("Error display sort menu! {0}")]
    SortMenu(String),
}
