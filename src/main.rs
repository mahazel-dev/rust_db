use std::collections::HashMap;


fn main()   {
    let mut employee_list   =   employee_list::create_HashMap();
    let mut command_buffor = input_command();

    println!("Before <extract command>{:?}", &command_buffor);
    println!("Command: {:?}", extract_command(&mut command_buffor));
    println!("----\nBefore AddEmployee{:?}", &command_buffor);
    AddEmployee(&mut employee_list, &mut command_buffor);
    println!("----\nAfter AddEmployee{:?}", &command_buffor);
    //println!("{:?}", AddEmployee(&mut employee_list, &mut command_buffor));

    //printList(&employee_list, &command_buffor);
    //println!("{:?}", extract_command(&commands));

}




mod employee;
use employee::*;

mod commands;
mod employee_list;

use commands::*;

