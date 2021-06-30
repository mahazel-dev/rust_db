use text_io::read;
use crate::employee::*;
use std::collections::HashMap;
use std::num::ParseIntError;


#[derive(Debug, PartialEq)]
pub enum Command    {
    Add,
    Print,
    Edit,
    Remove,
    Exit,
    Pass,
}


/*
pub fn printList(map: &HashMap<u32, Employee>, command: Ve) {
    let StringVec = StringToVec(command);
    match StringVec.get(1)  {
        Some(feature)    => {
            match *feature {
                "company" | "all" => println!("{:#?}", map),
                _ => for (key, value) in map.into_iter() {
                    if feature == map.Department { println!("ID: {}, {:#?}", key, value); }
                }
            }
        }
        None    => println!("Precise what to print <print department/all/company>")
    }
}
 */

fn StrToU32Handle(input: String) -> Result<u32, ParseIntError>   {
    let ID = input.to_string().parse::<u32>();
    ID
}


// *****************
pub fn EditEmployee(map: &mut HashMap<u32, Employee>, command: Vec<String>) {
    let mut buffor = command.clone();


    let ID: Result<u32, ParseIntError>;
    match buffor.pop() {
        None            =>  {   println!("What ID?");
            ID = StrToU32Handle(read!()); }
        Some(id)    =>  {  ID = StrToU32Handle(id);  }
    }

    match &ID    {
        Ok(_t)   => println!("Przeszlo"),
        Err(_e)  => {    println!("Wrong ID");
            return
        },
    }

    let ID = ID.unwrap();

    let positionID = map.get(&ID);
    match positionID {
        Some(n) => {
            println!("ID: {}, {:?}", &ID, n);

            println!("What part?");
            let column: String = read!();

            println!("To what?");
            let new_column: String = read!("{}\n");

            match &*column.to_lowercase() {
                "name" => { map.get_mut(&ID).unwrap().Name = new_column;  },
                "department" => {   map.get_mut(&ID).unwrap().Department = new_column; },
                _ => println!("Didn't find column")
            };
        },
        None => println!("ID not found :("),
    }
}


// *****************
pub fn AddEmployee(map: &mut HashMap<u32, Employee>, command: Vec<String>)    {
    let mut buffor = command.clone();
    let department = buffor.pop();

    buffor.pop();

    let name_op = buffor.get(..);

    let mut name = String::new();
    match department    {
        Some(dept)  =>  match name_op   {
            Some(value) => {
                for cell in value { name = name + &cell[..] + " ";}
                name.pop();
                map.insert(find_maxID(map) + 1, Employee::new(&name, &dept));
            }
            None    => return,
        },
        None    => return,
    };
}

// *****************
pub fn extract_command(input_vecstring: &mut Vec<String>) -> Command  {
    let extracted_command = input_vecstring.get(0);
    let output: Command;
    match extracted_command   {
        Some(command)   => {    match &command.to_lowercase()[..]   {
            "add"               =>  output = Command::Add,
            "print"             =>  output = Command::Print,
            "edit"              =>  output = Command::Edit,
            "delete" | "remove" =>  output = Command::Remove,
            "exit"              =>  output = Command::Exit,
            _                   =>  output = Command::Pass,}},
        None    => output = Command::Pass,
    }
    if output   !=  Command::Pass   {  input_vecstring.remove(0);  }
    output
}

// *****************
pub fn input_command()    -> Vec<String>   {         // String
    println!("Command: ");
    let buffor: String = read!("{}\n");
    let vec: Vec<String> = buffor.split_whitespace().map(str::to_string).collect();
    vec
}

// *****************
fn find_maxID   (map: &HashMap<u32, Employee>)   -> u32  {
    *map.keys().max().unwrap()
}

