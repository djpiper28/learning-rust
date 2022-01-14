use std::fs;
use std::io;

fn main() {
    println!("Insert a file name");
    let mut filename: String = String::new();
    io::stdin().read_line(&mut filename).expect("eof");

    let mut chars = filename.chars();
    chars.next_back();
    filename = chars.as_str().to_string();
   
    println!("Reading '{}'", filename);
        
    let contents = fs::read_to_string(filename).expect("cannot read file");
    println!("'{}'", contents);
}
