#[derive(Debug)]
enum Payment {
    Money(f32),
    DebitCard(bool, f32),
    _CreditCard(bool, f32)
}


fn main() {
    let payment_method = Payment::DebitCard(true, 100 as f32);

    match payment_method {
        Payment::Money(value) => println!("Now paying using '{:?} // Value: {}'", Payment::Money(120f32), value),
        Payment::DebitCard(true, value) => println!("Now paying using '{:?}' // Value: {}", Payment::DebitCard(true, 120f32), value),
        Payment::DebitCard(false, _) => println!("Error while trying to pay"),
        _ => {}
        
        
    }

}
