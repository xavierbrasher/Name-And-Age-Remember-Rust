#![allow(non_snake_case)]
use std::io::Write;
use std::fs;

use std::io::{stdin, stdout, Read};

// FIXME: fix pause 
fn pause() {
    println!("Press ENTER to continue...");
    let buffer = &mut [0u8];
    stdin().read_exact(buffer).unwrap();
    stdout().flush().unwrap();
    
}

fn clear() {
    println!("\x1B[2J\x1B[1;1H");
}

fn file_exists(file_name: String) -> bool {
    std::path::Path::new(&file_name).is_file()
} 

// TODO: finish makeFile
fn makeFile() {
    
}

// TODO: finish savefile
fn saveFile() {

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
        if x == (input.len() - 2) {
            break;
        }
        tmp.push(tmp_chars.nth(0).unwrap());
    }

    // returns the modified tmp
    return tmp;
}

fn listNames(name: &mut Vec<String>,  age: &mut Vec<String>) {
    for x in 0..name.len() {
        println!("{:?}: Name: {}, Age: {}", x+1, name[x], age[x])
    }
    pause();
}

fn addName(name: &mut Vec<String>,  age: &mut Vec<String>) {

    println!("Type q to cancel adding a name");


    let x: String = inputPr(String::from("Hello, User. What is your name: "));
    if x.len() == 1 && x.to_lowercase() == String::from("q") {
        return;
    }

    let y: String = inputPr(String::from(format!("Okay {}, Whats your age: ", x)));
    if y.len() == 1 && y.to_lowercase() == String::from("q") {
        return;
    }
    name.push(x);
    age.push(y);

    saveFile();
}

fn chooser(name: &mut Vec<String>, age: &mut Vec<String>) {
    // Getting names that are abled to be used

    // Printing chose and getting the responce
    clear();
    println!("Name and age remember \nType n to put someone in \nType r to remove a name \nType l to list out the names \nType q to quit");
    let responce: String = inputPr(String::from("Command: "));
    if responce.len() > 1 { 
        println!("{:?}", name);
        chooser(name, age); 
    }

    match responce.as_str() {
        "n" => {
            clear();
            println!("running add name"); // TODO: remove debug
            addName(name, age);
            chooser(name, age);
        },
        "l" => {
            clear();
            println!("running list names"); // TODO: remove debug
            listNames(name, age); 
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
    let (mut name, mut age): (Vec<String>, Vec<String>) = readfile();
    chooser(&mut name, &mut age);
    println!("Name: {:?}, Age: {:?}", name, age) // TODO: remove debug
}
