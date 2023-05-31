struct User {
    username: String,
    email: String,
    active: bool,
    gender: String,

}





fn main() {
    let mut person = User {username:String::from("placeholder"),
    
    email:String::from("gg@"), active:false, gender:String::from("b")};
    person.active = true; 

    println!(": {} | {} | {} | {}", person.username, person.email, person.active, person.gender);

    
}
