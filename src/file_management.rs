// on main rust file remember to put
// mod file_management;
// use file_management::*;
pub use std::fs;
pub use std::io::Read;

pub fn make_file(file_name : String) {
    match fs::write(file_name, ",,,,") {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e)
        }
    }
}

pub fn save_file(name: &mut Vec<String>, age: &mut Vec<String>, file_name : String) {
    if name.len() != age.len() {
        make_file("savedData.dat".to_string())
    }
    let mut contents: String = String::new();
    for x in 0..name.len() {
        let added: String = name[x].to_string() + "\n";
        contents += &added
    }
    contents += ",,,,\n";
    for x in 0..age.len() {
        let added: String = age[x].to_string() + "\n";
        contents += &added
    }
    match fs::write(file_name, contents) {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e)
        }
    }

}

pub fn file_exists(file_name: String) -> bool {
    //checks if the file exists by using .is_file
    std::path::Path::new(&file_name).is_file()
} 

pub fn vector_each_line(lines_to_be_split: String) -> Vec<String> {
    //comverts whole document to chars
    let mut chars = lines_to_be_split.chars();
    // creates a vector 
    let mut words: Vec<String> = Vec::new();
    //makes a tmp to remember
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

pub fn read_file() -> (Vec<String>, Vec<String>) {
    // sets a filename and checks if it exists 
    let result: bool = file_exists(String::from("savedData.dat"));
    if !result {
        make_file(String::from("savedData.dat"));
        return (Vec::new(), Vec::new())
    }

    // if not it will read the file and try to get it
    let mut f = fs::File::open(String::from("savedData.dat")).expect("Failed to open");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("could not read it");
    let lines: Vec<String> = vector_each_line(s);
    let (mut name, mut age) : (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
    let mut passed_commas: bool = false;
    for x in 0..lines.len() {
        if passed_commas == true {
            age.push(lines[x].to_string())
        }
        else if lines[x] == ",,,,".to_string() {
            passed_commas = true
        }
        else {
            name.push(lines[x].to_string())
        }
    }
    if name.len() == age.len() {
        (name, age)
    }
    else {
        make_file(String::from("savedData.dat"));
        (Vec::new(), Vec::new())
    }
    
}