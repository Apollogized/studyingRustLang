fn main() {
    //let integers = [1, 2, 3, 4, 5];
    let integers = [9;99];

    //for num in 0..integers.len(){
        //println!(": {}", integers[num]);
    //}

    for num in integers.iter(){
        println!(": {}", num);
    }
}
