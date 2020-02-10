use std::io;

#[derive(Debug)]
enum Command {
    Add(String, String), // Add X to Y
    PrintDepartment(String), // Print department X
    PrintAll, // Print all
    Quit, // Quit
}

fn parse(line: String) -> Option<Command> {
    let words: Vec<_> = line.trim().split_whitespace().collect();
    if words.len() >= 2 && words[0] == "Add" {
        if let Some(to_pos) = words.iter().position(|&w| w == "to") {
            let employee = words[1..to_pos].join(" ");
            let department = words[to_pos+1..].join(" ");
            Some(Command::Add(employee, department))
        } else {
            None
        }
    }
    else if words.len() >= 3 && words[0] == "Print" && words[1] == "department" {
        let department = words[2..].join(" ");
        Some(Command::PrintDepartment(department))
    }
    else if words.len() == 2 && words[0] == "Print" && words[1] == "all" {
        Some(Command::PrintAll)
    }
    else if words.len() == 1 && words[0] == "Quit" {
        Some(Command::Quit)
    }
    else {
        None
    }
}

fn main() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Failed to read line");
        
        if let Some(command) = parse(line) {
            match command {
                Command::Add(employee, department) => println!("Add \"{}\" to \"{}\"", employee, department),
                Command::PrintDepartment(department) => println!("Print department \"{}\"", department),
                Command::PrintAll => println!("Print all"),
                Command::Quit => break,
            }
        } else {
            println!("Wrong format")
        }
    }
}
