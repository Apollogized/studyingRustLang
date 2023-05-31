fn main() {
    let mut vectors = vec![1, 2, 3, 4];
    //let mut vectors2: Vec<i32> = Vec::new();

    println!(": {}", vectors[0]);
    vectors.push(5);
    println!(": {}", vectors[4]);
    vectors.remove(0);
    println!(": {:?}", vectors);
    
    for value in vectors.iter() {
       println!("// {}", value); 
    }
}
