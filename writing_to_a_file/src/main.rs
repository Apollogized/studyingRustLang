use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file_hand = File::create("test.txt")
        .expect("Couldn't create file");
    file_hand.write_all(b"Testing")
        .expect("Couldn't edit file");
}
