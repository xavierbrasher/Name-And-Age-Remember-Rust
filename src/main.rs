mod default_libary;
use default_libary::*;
mod file_management;
use file_management::*;

fn list_names(name: &mut Vec<String>,  age: &mut Vec<String>) {
    // repeats until all names and ages are shown 
    for x in 0..name.len() {
        println!("{:?}: Name: {}, Age: {}", x+1, name[x], age[x])
    }
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
    save_file(); // TODO: add name and age into requirements 
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
            println!("running add name"); // TODO: remove debug
            add_name(name, age);
            chooser(name, age);
        },
        "l" => {
            clear();
            println!("running list names"); // TODO: remove debug
            list_names(name, age); 
            chooser(name, age);
        }
        "q" => {
            clear();
            println!("Goodbye!")
        },
        _ => chooser(name, age),
    }

    
}

fn main() {
    // Reading file and setting variables for it
    let (mut name, mut age): (Vec<String>, Vec<String>) = read_file();
    chooser(&mut name, &mut age); 
    println!("Name: {:?}, Age: {:?}", name, age) // TODO: remove debug
}
