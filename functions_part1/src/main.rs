fn twice(num :i32) -> i32 {
    return 2*num;
}

fn max (a: i32, b:i32) -> i32 {
    if a >= b {
        return a;
    } else {
        return b;
    }
}


fn main() {
    println!(": {}", twice(222));
    println!(": {}", max(22, 40));
}
