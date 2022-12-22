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

    pub fn get_title(&self) -> &str {
        &self.title 
    }

    pub fn get_description(&self) -> &str{
        &self.description
    }

    pub fn set_title(&mut self, title:String){
       self.title = title; 
    }

    pub fn set_description(&mut self, description:String){
        self.description = description;
    }
}

