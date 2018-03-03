/*
    Employee Record System
    ~ (Rust Lang Exploration)
*/

use std::io::{
    stdin,
    stdout,
    Write
};

fn main() {

    let mut employees_id_index: i32 = 0;
    let mut employees: Vec<Employee> = Vec::new();

    loop {
        println!("--- EMPLOYEE RECORD ---");
        println!("MAIN MENU:");
        println!("1 - Show All Employees");
        println!("2 - Show Single Employee");
        println!("3 - Add Employee");
        println!("4 - Edit Employee");
        println!("5 - Delete Employee");
        println!("0 - Exit Program");

        let menu_input = {
            let str_input = String::from(
                take_input("> ")
            );
            if str_input.parse::<i8>().is_ok() {
                str_input.parse::<i8>().unwrap()
            } else {
                continue;
            }
        };

        match menu_input {
            1 => show_all_employees(&mut employees),
            2 => show_employee(&mut employees),
            3 => add_employee(&mut employees, &mut employees_id_index),
            4 => edit_employee(&mut employees),
            5 => delete_employee(&mut employees),
            0 => return,
            _ => continue,
        }
    }
}

struct Employee {
    id: i32,
    name: String,
    email: String,
    phone: String,
    address: String,
    job_desc: String,
    salary: f32,
}

fn show_one_employee(employees: &mut Vec<Employee>, index: usize) {
    println!("--- RECORD OF | {} | ---", employees[index].name.to_uppercase());
    println!("ID:\t\t {}", employees[index].id);
    println!("Name:\t\t {}", employees[index].name);
    println!("Email:\t\t {}", employees[index].email);
    println!("Phone:\t\t {}", employees[index].phone);
    println!("Address:\t {}", employees[index].address);
    println!("Job Desc:\t {}", employees[index].job_desc);
    println!("Salary:\t\t {}", employees[index].salary);
}

fn press_any_key() {
    take_input("Press any key to continue ...");
    println!("");
}

fn take_input(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}", prompt);

    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    return s;
}

fn search_employee_by_id(employees: &mut Vec<Employee>) -> (bool, usize) {
    #![allow(unused_assignments)]
    let mut id: i32 = 0;
    loop {
        let id_input = String::from(
            take_input("Enter Employee ID: ")
        );
        if id_input.parse::<i32>().is_ok() {
            id = id_input.parse::<i32>().unwrap();
            break;
        } else {
            println!("{}", "Invalid Value. Try Again!");
            continue;
        }
    }

    let mut index = 0 as usize;
    let mut is_emp_found = false;
    for (v_index, employee) in employees.iter().enumerate() {
        if employee.id == id {
            index = v_index;
            is_emp_found = true;
        }
    }
    
    return (is_emp_found, index);
}

fn show_all_employees(employees: &mut Vec<Employee>) {
    println!("");
    println!("-- ALL EMPLOYEES --");
    println!("");

    for employee in employees.iter() {
        let mut emp_to_display = Employee{
            id: employee.id.clone(),
            name: employee.name.clone(),
            email: employee.email.clone(),
            phone: employee.phone.clone(),
            address: employee.address.clone(),
            job_desc: employee.job_desc.clone(),
            salary: employee.salary.clone(),
        };

        println!(
            "| ID: {} | NAME: {} |\n| EMAIL: {} | PHONE: {} |\n| ADDRESS: {} |\n| JOB DESC: {} | SALARY: {} |",
            emp_to_display.id,
            emp_to_display.name,
            emp_to_display.email,
            emp_to_display.phone,
            emp_to_display.address,
            emp_to_display.job_desc,
            emp_to_display.salary,
        );

        println!("----------------------------------------------------------------");
    }

    println!("");
    println!("-- ALL EMPLOYEES --");
    println!("");
    press_any_key();
}

fn show_employee(employees: &mut Vec<Employee>) {
    println!("");
    println!("-- SHOW EMPLOYEE --");
    println!("");

    println!("Enter ID of Employee you want to see.");
    let is_emp_found = search_employee_by_id(employees);

    if is_emp_found.0 {
        println!("\nEmployee Found!\n");
        show_one_employee(employees, is_emp_found.1 as usize);
        println!("\nEmployee Found!\n");
        press_any_key();
    } else {
        println!("\nEmployee Not Found!\n");
        press_any_key();
    }
}

fn add_employee(employees: &mut Vec<Employee>, employees_id_index: &mut i32) {
    println!("");
    println!("-- ADD EMPLOYEE --");
    println!("");

    let mut new_employee = Employee {
        id: -1,
        name: String::from(
            take_input("Employee Name: ")
        ),
        email: String::from(
            take_input("Employee Email: ")
        ),
        phone: String::from(
            take_input("Employee Phone: ")
        ),
        address: String::from(
            take_input("Employee Address: ")
        ),
        job_desc: String::from(
            take_input("Employee Job Description: ")
        ),
        salary: 0.0,
    };

    loop {
        let salary = String::from(
            take_input("Employee Salary e.g. 10.75: ")
        );
        if salary.parse::<f32>().is_ok() {
            new_employee.salary = salary.parse::<f32>().unwrap();
            break;
        } else {
            println!("{}", "Invalid Value. Try Again!");
            continue;
        }
    }

    new_employee.id = *employees_id_index + 1;
    (*employees_id_index) += 1;

    employees.push(new_employee);
    let new_emp_id = employees.len() - 1;
    println!("\nNew Employee Added!\n");
    show_one_employee(employees, new_emp_id);
    println!("\nNew Employee Added!\n");
    press_any_key();
}

fn edit_employee(employees: &mut Vec<Employee>) {
    println!("");
    println!("-- EDIT EMPLOYEE --");
    println!("");

    println!("Enter ID of Employee you want to edit.");
    let is_emp_found = search_employee_by_id(employees);

    if is_emp_found.0 {
        println!("\nEmployee Found!\n");
        show_one_employee(employees, is_emp_found.1 as usize);
        println!("\nEmployee Found!\n");
    
        loop {
            println!("EDIT MENU:");
            println!("1 - Edit Name");
            println!("2 - Edit Email");
            println!("3 - Edit Phone");
            println!("4 - Edit Address");
            println!("5 - Edit Job Desc");
            println!("6 - Edit Salary");
            println!("0 - Return To Main Menu");

            let menu_input = {
                let str_input = String::from(
                    take_input("> ")
                );
                if str_input.parse::<i8>().is_ok() {
                    str_input.parse::<i8>().unwrap()
                } else {
                    continue;
                }
            };

            fn confirm_edit(employees: &mut Vec<Employee>, index: usize) {
                println!("\nEdited Employee!\n");
                show_one_employee(employees, index);
                println!("\nEdited Employee!\n");
                press_any_key();
            }

            match menu_input {
                1 => {
                    employees[is_emp_found.1].name = String::from(
                        take_input("Employee Name: ")
                    );
                    confirm_edit(employees, is_emp_found.1);
                },
                2 => {
                    employees[is_emp_found.1].email = String::from(
                        take_input("Employee Email: ")
                    );
                    confirm_edit(employees, is_emp_found.1);
                },
                3 => {
                    employees[is_emp_found.1].phone = String::from(
                        take_input("Employee Phone: ")
                    );
                    confirm_edit(employees, is_emp_found.1);
                },
                4 => {
                    employees[is_emp_found.1].address = String::from(
                        take_input("Employee Address: ")
                    );
                    confirm_edit(employees, is_emp_found.1);
                },
                5 => {
                    employees[is_emp_found.1].job_desc = String::from(
                        take_input("Employee Job Description: ")
                    );
                    confirm_edit(employees, is_emp_found.1);
                },
                6 => 'salary_loop: loop {
                        let salary = String::from(
                            take_input("Employee Salary e.g. 10.75: ")
                        );
                        if salary.parse::<f32>().is_ok() {
                            employees[is_emp_found.1].salary = salary.parse::<f32>().unwrap();
                            confirm_edit(employees, is_emp_found.1);
                            break 'salary_loop;
                        } else {
                            println!("{}", "Invalid Value. Try Again!");
                            continue 'salary_loop;
                        }
                    },
                0 => {
                    println!("");
                    return;
                },
                _ => continue,
            }
        }
    } else {
        println!("\nEmployee Not Found!\n");
        press_any_key();
    }
}

fn delete_employee(employees: &mut Vec<Employee>) {
    println!("");
    println!("-- DELETE EMPLOYEE --");
    println!("");

    println!("Enter ID of Employee you want to delete.");
    let is_emp_found = search_employee_by_id(employees);

    if is_emp_found.0 {
        'ask_deelete: loop {
            println!("\nEmployee To Be Deleted!\n");
            show_one_employee(employees, is_emp_found.1);
            println!("\nEmployee To Be Deleted!\n");

            let confirm_delete = String::from(
                take_input("Press 'y' to delete or 'n' to return to Main Menu: ")
            );
            match confirm_delete.to_lowercase().as_ref() {
                "y" => {
                    employees.remove(is_emp_found.1);
                    println!("\nEmployee Deleted!\n");
                    press_any_key();
                    return;
                },
                "n" => {
                    return;
                },
                _ => {
                    continue 'ask_deelete;
                }
            }
        }
    } else {
        println!("\nEmployee Not Found!\n");
        press_any_key();
    }
}
