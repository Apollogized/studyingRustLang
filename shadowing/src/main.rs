fn main() {
    let a: i32 = 10;

    {
        println!("a: {}", a);
        let a: f32 = 20.002;
        println!("a is now: {}", a);
    }
    println!("a returns to: {}", a);
}
