use std::collections::HashMap;


fn main()   {
    let mut employee_list   =   employee_list::create_HashMap();
    let command_buffor = input_text();
    printList(&employee_list, &command_buffor);
    //println!("{:?}", extract_command(&commands));

}




mod employee;
use employee::*;

mod commands;
mod employee_list;

use commands::*;

