fn main() {
    let tuple = (42, "hello", 3.14, false, (1, 2, 3));
    let (a, b, c, d, e) = tuple;
    
    println!("a: {}, b: {}, c: {}, d: {}, e: {:?}", a, b, c, d, e);
}
