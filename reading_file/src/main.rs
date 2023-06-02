use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut fileh = File::open("rust_wiki.txt").expect("Couldn't open file");
    let mut content = String::new();

    fileh.read_to_string(&mut content).expect("Couldn't read file");

    println!("File content: \n\n{}", content);
    
}
