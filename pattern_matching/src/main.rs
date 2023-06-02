fn main() {
    let num = 3;

    match num {
        1 => println!(": {}", num),
        2 | 3 => println!(": {}", num),
        //4..10 => println!(": "),
        _ => println!("Number {} not included on match", num)
    }
}
