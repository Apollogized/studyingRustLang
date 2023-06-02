use std::io;

fn main() {
    let mut user_message = String::new();

    println!("Type anything: ");
    
    match io::stdin().read_line(&mut user_message) {
        Ok(_) => println!("OK! You typed: {}", user_message.to_uppercase()),
        Err(e) => println!("ERR! Something went wrong.. // {},", e)
    }
}
