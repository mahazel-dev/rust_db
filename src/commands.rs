use text_io::read;
use crate::employee::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Command    {
    Add,
    Print,
    Edit,
    Remove,
    Exit,
    Pass,
}


pub fn AddEmployee(map: &mut HashMap<u32, Employee>, string: &String)    {
    let StringVec = StringToVec(string);

    let department  =

    let StringVec = StringToVec(string);
    let TupleData = Add_ExtractToTuple(&StringVec);

    let person  =   Employee    {
        Name:   TupleData.1,
        Department: TupleData.2,
    };
    let index = find_maxID(&map) + 1;

    map.insert(index, person);

}

pub fn input_text()    -> String   {
    println!("Command: ");
    let mut buffor: String = read!("{}\n");
    buffor
}

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

pub fn extract_command(input_string: &mut String)   -> Command  {
    let extracted_command= StringToVec(input_string); //.get(0);
    let extracted_command = extracted_command.get(0);
    match extracted_command   {
        Some(command)   => {    match &command.to_lowercase()[..]   {
            "add"               =>  Command::Add,
            "print"             =>  Command::Print,
            "edit"              =>  Command::Edit,
            "delete" | "remove" =>  Command::Remove,
            "exit"              =>  Command::Exit,
            _                   =>  Command::Pass,
        }
        },
        None    => Command::Pass,
    }
}

fn StringToVec(inputText: &String)  ->  Vec<&str>   {
    let vector: Vec<&str> = inputText.split(" ").collect();
    vector
}

fn find_maxID   (map: &HashMap<u32, Employee>)   -> u32  {
    *map.keys().max().unwrap()
}