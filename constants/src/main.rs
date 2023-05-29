const PI_NUMBER: f32 = 3.14159;

fn circumference_length(r: f32) -> f32 {
    let math = 2 as f32*r*PI_NUMBER;

    math
}

fn main() {
    println!(": {}", circumference_length(1 as f32)); // or just 1.0
}
