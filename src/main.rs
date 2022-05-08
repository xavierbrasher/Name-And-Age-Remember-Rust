#![allow(non_snake_case)]
use std::io::Write;
use std::fs;

fn file_exists(file_name: String) -> bool {
    std::path::Path::new(&file_name).is_file()
} 

fn makeFile() {
    
}

fn vector_each_line(words_to_be_split: String) -> Vec<String> {
    let mut chars = words_to_be_split.chars();
    let mut words: Vec<String> = Vec::new();
    let mut tmp: String = String::new();
    for _x in 0..chars.as_str().len() {
        let each_char: char = chars.nth(0).unwrap();
        if each_char == '\n' {
            words.push(tmp);
            tmp = String::new();
        } else if each_char == '\r' {
            continue;
        } else {
            tmp.push(each_char);
        }
    }
    //println!("{:?}", vec);
    return words;
}

fn readfile() -> (Vec<String>, Vec<String>) {
    // sets a filename and checks if it exists 
    let file_name: String = String::from("savedData.dat");
    let result: bool = file_exists(file_name);
    if !result {
        makeFile();
        return (Vec::new(), Vec::new())
    }

    // if not it will read the file and try to get it
    let mut f = fs::read("savedData.dat").expect("Error occured");
    
    (Vec::new(), Vec::new())
}

fn input() -> String {
    // gets stdin and sets to buffer
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    // get tmps and removes the last 2 characters of it    
    let mut tmp: String = String::new();
    let mut tmp_chars = input.chars();
    for x in 0..input.len() {
        if x == input.len() - 2 {
            break;
        }
        tmp.push(tmp_chars.nth(0).unwrap());
    }
    // returns the modified tmp
    return tmp;
}

fn inputPr(content: String) -> String {
    // prints a single line without \n and flushes the stdout
    print!("{}", content);
    std::io::stdout().flush().unwrap();

    // gets stdin and sets to buffer
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    // get tmps and removes the last 2 characters of it    
    let mut tmp: String = String::new();
    let mut tmp_chars = input.chars();
    for x in 0..input.len() {
        if x == input.len() - 2 {
            break;
        }
        tmp.push(tmp_chars.nth(0).unwrap());
    }

    // returns the modified tmp
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
    // Getting names that are abled to be used
    let name: Vec<String> = nameF;
    let age: Vec<String> = ageF;

    // Printing chose and getting the responce
    println!("Name and age remember \nType n to put someone in \nType r to remove a name \nType l to list out the names \nType q to quit");
    let responce: String = inputPr(String::from("Command: "));
    if responce.len() > 1 { chooser(name, age); }

    
}

fn main() {
    // Reading file and setting variables for it
    let (name, age): (Vec<String>, Vec<String>) = readfile();
    chooser(name, age);
}
