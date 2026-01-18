use std::collection::HashMap
use std:io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop{
        println!("\n Enter a command: ");
        println!("\n add <name> to Department ");
        println!("3. list <department>");
        println!("list all");
        println!("4. exist");


        let mut input String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exist"){
            break;
        }
        else if input.to_lowercase().start_with("add"){
            add_employee(input, &mut company);
        }
        else if input.to_lowercase().start_with("list all"){
            list_all(&company);

        }
        else if input.to_lowercase().start_with("list"){
            list_department(input, &company);
        }
        else{
            println!("invalida command");
        }
    }
}



// function fo the add employees
fn add_employee(command:&str, company: &mut HashMap<String, Vec<String>>){
    let parts:Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 4 {
        println!("invalida format use Add <name> to <Department>");
        return
    }

    let name = parts[1].to_string();
    let department = parts[3].to_string();


    company
        .entry(department)
        .or_insert(Vec::new())
        .push(name);

    println!("Employe added successfully");
}

