use std::collections::HashMap;

fn main() {
    let mut hash_map = HashMap::new();
    hash_map.insert("a", 1);
    hash_map.insert("b", 3);
    hash_map.insert("c", 5);
    //hash_map.insert("d", 7);
    
    println!("Length of 'hash_map': {}", hash_map.len());

    match hash_map.get("d"){
        Some(value) => println!("'d' is on 'hash_map' with the value: {}", value),
        None => println!("'d' is not included on 'hash_map'") 
    }

    println!("Is 'b' included? {}", hash_map.contains_key("b"));
}
