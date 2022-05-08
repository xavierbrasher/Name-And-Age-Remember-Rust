#![allow(non_snake_case)]
use std::io::Write;
use std::fs;

fn file_exists(file_name: String) -> bool {
    std::path::Path::new(&file_name).is_file()
} 

fn makeFile() {
    
}

fn readfile() -> (Vec<String>, Vec<String>) {
    let file_name: String = String::from("savedData.dat");
    if !file_exists(file_name) {
        makeFile();
        return (Vec::new(), Vec::new())
        
    }
    let mut f = fs::read("savedData.dat").expect("Error occured");
    
    (Vec::new(), Vec::new())
}

fn input() -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    let mut tmp: String = String::new();
    let mut tmp_chars = input.chars();
    for x in 0..input.len() {
        if x == input.len() - 2 {
            break;
        }
        tmp.push(tmp_chars.nth(0).unwrap());
    }
    return tmp;
}

fn inputPr(content: String) -> String {
    print!("{}", content);
    std::io::stdout().flush().unwrap();
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    let mut tmp: String = String::new();
    let mut tmp_chars = input.chars();
    for x in 0..input.len() {
        if x == input.len() - 2 {
            break;
        }
        tmp.push(tmp_chars.nth(0).unwrap());
    }
    return tmp;
}

fn addName(nameF: Vec<String>,  ageF: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut name: Vec<String> = nameF;
    let mut age: Vec<String> = ageF;
    let mut on: bool = true;
    println!("Type q to quit");

    while on {
        let x: String = inputPr(String::from("Hello, User. What is your name: "));
        if x.len() == 1 && x.to_lowercase() == String::from("q") {
            on = false;
            continue;
        }

        let y: String = inputPr(String::from(format!("Okay {}, Whats your age: ", x)));
        if y.len() == 1 && y.to_lowercase() == String::from("q") {
            on = false;
            continue;
        }
        name.push(x);
        age.push(y);
    }
    (name, age)
}

fn chooser(nameF: Vec<String>, ageF: Vec<String>) {
    //Getting names that are abled to be used
    let name: Vec<String> = nameF;
    let age: Vec<String> = ageF;

    println!("Name and age remember \nType n to put someone in \nType r to remove a name \nType l to list out the names \nType q to quit");
    let responce: String = inputPr(String::from("Command: "));
    if responce.len() > 1 { chooser(name, age); }

    let mut nameS: Vec<String> = Vec::new();
    let mut ageS: Vec<String> = Vec::new();
    
}

fn main() {
    let name: Vec<String> = Vec::new();
    let age: Vec<String> = Vec::new();
    println!("{}", file_exists(String::from("savedData.dat")));
    chooser(name, age);
}
