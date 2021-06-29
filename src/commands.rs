use text_io::read;
use crate::employee::*;
use std::collections::HashMap;

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
pub fn printList(map: &HashMap<u32, Employee>, command: &String) {
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
    /*
        if StringVec.len() != 1 {
            let department = *StringVec.last().unwrap();
            match department    {
                "company" | "all"    => println!("{:#?}", map),
                _=> for (key, value) in map.into_iter()  {
                    if department == value.Department   {
                        println!("ID: {}, {:#?}", key, value)}
                },
            };
        }
     */
}
 */

pub fn AddEmployee(map: &mut HashMap<u32, Employee>, command_buffor: &mut Vec<String>)    {
    let department = command_buffor.pop();
    let dir = command_buffor.pop();
    let name_op = command_buffor.get(..);

    let mut name = String::new();

    for cell in name_op.unwrap() {
        name = name + &cell[..] + " ";
    }
    name.pop();

    println!("Name {:?}", name);
    //println!("Buffer in function {:?}", command_buffor);
    /*
    match department    {
        Some(dept)  =>  {
            match dir {
                Some(d) =>  {
                    match name  {
                        Some(n) =>  {   for word  in n   {
                            _name = _name + &word[..] + &" ";
                        }},
                        None    => (),
                    };
                },
                None    => (),
            };
        },
        None    => (),
    };
     */

    //if department == None || dir == None || name == None {


    name.pop();
}

    //else {println!("Duper duper")};
    /*
    let person  =   Employee    {
        Name:   TupleData.1,
        Department: TupleData.2,
    };
    let index = find_maxID(&map) + 1;
    map.insert(index, person);
     */


pub fn extract_command(input_string: &mut Vec<String>)   -> Command  {
    //let extracted_command= StringToVec(input_string)
    let extracted_command = input_string.get(0);
    let mut output: Command;
    match extracted_command   {
        Some(command)   => {    match &command.to_lowercase()[..]   {
            "add"               =>  output = Command::Add,
            "print"             =>  output = Command::Print,
            "edit"              =>  output = Command::Edit,
            "delete" | "remove" =>  output = Command::Remove,
            "exit"              =>  output = Command::Exit,
            _                   =>  output = Command::Pass,}
        },
        None    => output = Command::Pass,
    }
    if output   !=  Command::Pass   {  input_string.remove(0);  }
    output
}

pub fn input_command<'a>()    -> Vec<String>   {         // String
    println!("Command: ");
    let mut buffor: String = read!("{}\n");
    let vec: Vec<String> = buffor.split_whitespace().map(str::to_string).collect();
    vec
}

fn find_maxID   (map: &HashMap<u32, Employee>)   -> u32  {
    *map.keys().max().unwrap()
}