fn main() {
    let mut myString: String = String::from("Thissussy raw text is &str, but is being converted to String");

    println!("My String size: {}", myString.len());
    println!("Is my String empty? {}", myString.is_empty());
    
    for word in myString.split_whitespace() {
        println!("{}", word);

    }

    println!("String contains sussy? {}", myString.contains("sussy"));
    myString.push_str("Added with push_str");
    println!(": {}", myString);
    
}
