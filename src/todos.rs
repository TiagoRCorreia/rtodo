pub struct Todo{
   pub title: String,
   pub description: String,
}

impl Todo{

    pub fn new(title:String,description:String) -> Self{
        Self{
            title,
            description,
        }
    }
}

