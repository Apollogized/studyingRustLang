#[derive(Debug)]

enum Payment {
    Money,
    CreditCard,
    DebitCard
}

fn main() {
    let payment_method = Payment::DebitCard;

    match payment_method {
        Payment::CreditCard => println!("Payment method is now '{:?}'", Payment::CreditCard),
        Payment::DebitCard => println!("Payment method is now '{:?}'", Payment::DebitCard),
        _ => {}
    }

}
