struct User {
    username: String,
    email: String,
    gender: String,
    active: bool
}

impl User {
    fn name_user(&self){
        println!("Your username is: {}", self.username);
    }

    fn active_or_offline(&self){
        println!("User is active? {}", self.active);
    }
}


fn main() {
    let profile = User 
    {
        username:String::from("Username123"),
        email:String::from("email@mail.com"),
        gender:String::from("something"),
        active:false

    };

    println!
    (
        "{} | {} | {} | {}",
        profile.username,
        profile.email,
        profile.gender,
        profile.active

    );
    profile.name_user();
    profile.active_or_offline();

}
