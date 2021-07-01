
fn main() {
    let mut employee_list = employee_list::create_HashMap();

    'command_loop: loop {
        let mut command_buffor = input_command();
        match extract_command(&mut command_buffor) {
            Command::Add    =>  AddEmployee(&mut employee_list, command_buffor),
            Command::Edit   =>  EditEmployee(&mut employee_list, command_buffor),
            Command::Print  =>  FilterEmployeeList(&employee_list, command_buffor),
            Command::Exit   =>  break 'command_loop,
            _               =>  println!("{:?}", Command::Pass),
        };
        //println!("----\nYour list: {:#?}", &employee_list);
    }
}


mod employee;
use employee::*;

mod commands;
mod employee_list;

use commands::*;

