// on main rust file remember to put
// mod file_management;
// use file_management::*;
pub use std::fs;

// TODO: finish makeFile
pub fn make_file() {
    
}

// TODO: finish savefile
pub fn save_file() {

}

pub fn file_exists(file_name: String) -> bool {
    //checks if the file exists by using .is_file
    std::path::Path::new(&file_name).is_file()
} 

pub fn vector_each_line(words_to_be_split: String) -> Vec<String> {
    //comverts whole document to chars
    let mut chars = words_to_be_split.chars();
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
    let file_name: String = String::from("savedData.dat");
    let result: bool = file_exists(file_name);
    if !result {
        make_file();
        return (Vec::new(), Vec::new())
    }

    // if not it will read the file and try to get it
    let mut f = fs::read("savedData.dat").expect("Error occured");
    
    (Vec::new(), Vec::new())
}