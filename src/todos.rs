/// Struct to keep the todos
pub struct Todo{
   pub title: String,
   pub description: String,
}

impl Todo{
    // Constructor for initializing the Todo
    pub fn new(title:String,description:String) -> Self{
        Self{
            title,
            description,
        }
    }
}

