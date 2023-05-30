fn main() {
    let mut x = 10;
    let a = &mut x;
    *a += 1;

    // references are not immutable by standard


    println!(": {}", a);
}
