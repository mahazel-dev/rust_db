#[derive(Debug)]
pub struct Employee<'a> {
    pub Name: &'a str,
    pub Department: &'a str,
}

impl <'a>Employee<'a>   {
    pub fn new(name: &'a str, department: &'a str)    -> Self {
        Self    {   Name: name, Department: department,    }
    }
}


