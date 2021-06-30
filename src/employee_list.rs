use std::collections::HashMap;
use crate::employee::*;



pub fn create_HashMap() ->  HashMap<u32, Employee>  {
    let mut list: HashMap<u32, Employee> = HashMap::new();

    list.insert(000, Employee::new("Adam Kozlowski", "Sales"));
    list.insert(001, Employee::new("Kamila Rosinska", "Production"));
    list
}

