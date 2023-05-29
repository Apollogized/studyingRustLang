use std::io;

fn convert_to_int(data_input: & String) -> i32{
    let strto_sig32 = data_input.trim().parse::<i32>().unwrap();
    strto_sig32
}

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Error reading 'num'");
     
    let mut sum = 0;
    let mut strto_sig32 = convert_to_int(&num);
    while strto_sig32 !=0 {
       let res = strto_sig32 %10;
       sum += res;
       strto_sig32 = strto_sig32 /10;
    }
    println!("The final value is {sum}");

}

