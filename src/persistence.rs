use directories::BaseDirs;

use rtodo::todos::Todo;

// Read todos from file
pub fn read_from_file() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    // Get todos file
    let path = BaseDirs::new().unwrap().config_dir().join("rtodo/db.json");
    // Read todos from file as string
    let td_str = std::fs::read_to_string(path)?;

    // Create a vector of todos
    let todos: Vec<Todo> = serde_json::from_str(&td_str)?;

    // return todos
    Ok(todos)
}

// Write todos into file
pub fn write_to_file(todos: &Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    // Get .config directory on Linux system
    let path = BaseDirs::new().unwrap().config_dir().join("rtodo");

    // Create rtodo directory
    if !path.exists() {
        std::fs::create_dir(&path)?;
    }

    // Write todos into file
    std::fs::write(path.join("db.json"), serde_json::to_string(&todos)?)?;
    Ok(())
}
