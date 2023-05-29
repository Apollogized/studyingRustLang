use std::io; // use standard input and output

fn convert_to_int(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
    
}

fn main() {
   let mut num1 = String::new();
   io::stdin().read_line(&mut num1).expect("Error reading num1"); 

   let mut num2 = String::new();
   io::stdin().read_line(&mut num2).expect("Error reading num2"); 

   if convert_to_int(&num1) > convert_to_int(&num2){
       println!("{num1} is bigger than {num2}");
   } 
   else {
       println!("{num1} is lower or equal to {num2}");
   }
}
