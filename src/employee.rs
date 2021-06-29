#[derive(Debug)]
pub struct Employee {
    pub Name: String,
    pub Department: String,
}

impl Employee   {
    pub fn new(name: &str, department: &str)    -> Self {
        Self    {   Name: String::from(name), Department: String::from(department),    }
    }
}


