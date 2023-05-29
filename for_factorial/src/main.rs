use std::io;

fn convert_to_int(data_input: & String) -> i32 {
    let converted_data = data_input.trim().parse::<i32>().expect("ERROR");    
    return converted_data;
}

fn factorial(num: i32) -> i32 {
    let mut tmp = 1;

    for num in 1..(num+1){
        tmp *= num;
    }
    return tmp;
}

fn main() {
    let mut amount_input = String::new();
    io::stdin().read_line(&mut amount_input).expect("Error reading input");

    let amount_input_i32 = convert_to_int(&amount_input);

    for a in  0..amount_input_i32 {
        
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Error reading input");
        
        let user_input_i32 = convert_to_int(&user_input);
        let factorial_value = factorial(user_input_i32); 
        
        println!("The factorial of {} is {}", user_input_i32, factorial_value);

    } 


}
