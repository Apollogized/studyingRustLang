use std::io;

fn convert_to_int(data: & String) -> i32{
    let convert_data = data.trim().parse::<i32>().unwrap();
   
    convert_data
}

fn main() {
    let mut students_num_str = String::new();
    io::stdin().read_line(&mut students_num_str).expect("Error reading 'number'");
    let mut students_num_i32 = convert_to_int(&students_num_str);

    let mut exam_quantity = 0;

    while students_num_i32 >0{
        
        let mut grade_average_str = String::new();
        io::stdin().read_line(&mut grade_average_str).expect("ERROR"); 
        let grade_average_i32 = convert_to_int(&grade_average_str);

        if grade_average_i32 >=3 && grade_average_i32 <6 {
           
            exam_quantity += 1;
        }
        students_num_i32 -= 1; 
    } 
    println!("{exam_quantity} students are on exam.");
}
