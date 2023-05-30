static mut STATIC_VARIABLE: i32 = 12;

fn main() {

    unsafe {
        println!(": {}", STATIC_VARIABLE);
    }
    

}
