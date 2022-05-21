mod default_libary;
use default_libary::*;
mod file_management;
use file_management::*;

fn list_names(name: &mut Vec<String>,  age: &mut Vec<String>, pause_check: bool) {
    // repeats until all names and ages are shown 
    if name.len() != 0 {
        for x in 0..name.len() {
            println!("{:?}: Name: {}, Age: {}", x+1, name[x], age[x])
        }
    }
    else {
        println!("No names or ages in list")
    }
    if pause_check {
        pause();
    }
   
}

fn remove_name(name: &mut Vec<String>,  age: &mut Vec<String>) {
    if name.len() == 0 {
        println!("No names or ages in list");
        pause();
        return;
    }
    list_names(name, age, false);
    let responce : usize = match input_pr("Which one would you want to remove: ".to_string()).trim().parse() {
        Ok(num) => {num},
        Err(_) => {
            clear();
            println!("Please put in a number");
            pause();
            return;
        }
    };
    if responce > name.len() {
        clear();
        println!("That is out of range");
        pause();
        return;
    }
    name.remove(responce - 1);
    age.remove(responce - 1);
    clear();
    println!("It has been removed");
    pause();
}

fn add_name(name: &mut Vec<String>,  age: &mut Vec<String>) {

    println!("Type q to cancel adding a name");

    // gets users input name
    let x: String = input_pr(String::from("Hello, User. What is your name: "));
    if x.len() == 1 && x.to_lowercase() == String::from("q") { //checks if it equals to q and if len is 1
        return;
    }

    // gets users input age
    let y: String = input_pr(String::from(format!("Okay {}, Whats your age: ", x)));
    if y.len() == 1 && y.to_lowercase() == String::from("q") { //checks if it equals to q and if len is 1
        return;
    }

    // pushes it to the name and age vec
    name.push(x);
    age.push(y);

    //calls save and then returns
    
}

fn chooser(name: &mut Vec<String>, age: &mut Vec<String>) {
    save_file(name, age, String::from("savedData.dat"));
    // Getting names that are abled to be used

    // Printing chose and getting the responce
    clear();
    println!("Name and age remember \nType n to put someone in \nType r to remove a name \nType l to list out the names \nType q to quit");
    let responce: String = input_pr(String::from("Command: "));
    // if responces is larger than 1 it will redo
    if responce.len() > 1 { 
        println!("{:?}", name);
        chooser(name, age); 
    }

    // case check as it is more efficent than else if operations
    match responce.as_str() {
        "n" => {
            clear();
            add_name(name, age);
            chooser(name, age);
        },
        "l" => {
            clear();
            list_names(name, age, true); 
            chooser(name, age);
        }
        "r" => {
            clear();
            remove_name(name, age);
            chooser(name, age);
        },
        "q" => {
            clear();
            println!("Goodbye!");
            save_file(name, age, String::from("savedData.dat"));
        },
        _ => chooser(name, age),
    }

    
}

fn main() {
    // Reading file and setting variables for it
    let (mut name, mut age): (Vec<String>, Vec<String>) = read_file();
    chooser(&mut name, &mut age); 
    
}
