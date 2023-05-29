use std::io;

fn convert_to_int(data: & String) -> i32{
    let factdata_conv = data.trim().parse::<i32>().unwrap();

    factdata_conv
}

fn main() {
    let mut factorial_data = String::new();
    io::stdin().read_line(&mut factorial_data).expect("ERROR READING");
    
    let mut factorial = 1;
    let mut factdata_conv = convert_to_int(&factorial_data);
    while factdata_conv >1{
        factorial *= factdata_conv;
        factdata_conv -= 1;
    }
    println!("The factorial value is {factorial}");
}

