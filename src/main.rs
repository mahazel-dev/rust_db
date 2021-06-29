use std::collections::HashMap;


fn main()   {
    let mut employee_list   =   employee_list::create_HashMap();
    let mut command_buffor = input_command();

    match extract_command(&mut command_buffor)  {
        Command::Add    =>  AddEmployee(&mut employee_list, command_buffor),
        _               =>  println!("{:?}", Command::Pass),
    }
    //println!("Before <extract command>{:?}", &command_buffor);
    //println!("Command: {:?}", extract_command(&mut command_buffor));
    //println!("----\nBefore AddEmployee{:?}", &command_buffor);
    //AddEmployee(&mut employee_list, command_buffor);
    println!("----\nYour list: {:?}", &employee_list);
    //println!("{:?}", AddEmployee(&mut employee_list, &mut command_buffor));

    //printList(&employee_list, &command_buffor);
    //println!("{:?}", extract_command(&commands));

}




mod employee;
use employee::*;

mod commands;
mod employee_list;

use commands::*;

