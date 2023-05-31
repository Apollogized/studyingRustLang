#[derive(Debug)]
struct User {
    username: String,
    email: String,
    gender: String,
    active: bool
}

fn print_username(user_parameter: &User){
    println!("Your username is {}", user_parameter.username);
}

fn main() {
    let mut user_credentials = User 
    {
        username:String::from("Me"),
        email:String::from("myself@i.com"),
        gender:String::from("something"),
        active:false
    };
    user_credentials.active = false;

    println!
        (
            "{} | {} | {}",
            user_credentials.username,
            user_credentials.email,
            user_credentials.gender
        );
    print_username(&user_credentials);
    print_username(&user_credentials);    
}
