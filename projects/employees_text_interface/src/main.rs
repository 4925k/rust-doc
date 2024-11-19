use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    // hold company info
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    // loop to keep asking for user input
    loop {
        // display menu
        println!("\nMenu");
        println!("1. Add employee to department");
        println!("2. List all employees of a company");
        println!("3. List all employees of a department");
        println!("4. Exit");

        // parse user input
        print!("Choose your option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        // choose function based on choice
        match choice.trim() {
            "1" => {
                add_employee(&mut company);
            }
            "2" => {
                list_department(&mut company);
            }
            "3" => {
                list_employees(&mut company);
            }
            "4" => break,
            _ => println!("\nInvalid option selected."),
        }
    }
}

// ask user for employee and department and then add it on our backend
fn add_employee(company: &mut HashMap<String, Vec<String>>) {
    print!("Enter employee name: ");
    io::stdout().flush().unwrap();

    let mut name = String::new(); // mut inorder to trim whitespaces later on
    io::stdin().read_line(&mut name).unwrap();
    let name = &name.trim();

    print!("Enter department name: ");
    io::stdout().flush().unwrap();

    let mut department = String::new();
    io::stdin().read_line(&mut department).unwrap();
    let department = &department.trim();

    // add to existing department or create a new one if it doesn't exist
    company.entry(department.to_string()).or_insert(Vec::new()).push(name.to_string());

    println!("Added {} to {}", name, department);
}

fn list_department(company: &mut HashMap<String, Vec<String>>) {
    print!("Enter department name: ");
    io::stdout().flush().unwrap();

    let mut department = String::new();
    io::stdin().read_line(&mut department).unwrap();
    let department = department.trim();

    if let Some(employees) = company.get(department) {
        let mut sorted_employees = employees.clone();
        sorted_employees.sort();

        println!("Employees in {}", department);

        for e in sorted_employees {
            println!("  {}", e);
        }
    } else {
        println!("No such department found")
    }

}

fn list_employees(company: &mut HashMap<String, Vec<String>>) {
    let mut departments: Vec<_> = company.keys().collect();
    departments.sort();

    for d in departments {
        println!("{}", d);
        let mut sorted_employees = company.get(d).unwrap().clone();
        sorted_employees.sort();

        for e in sorted_employees {
            println!("  {}", e);
        }
    }
}