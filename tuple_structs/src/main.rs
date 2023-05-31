struct User (String, String, String, bool);

fn main() {
    let profile = User 
    (
        String::from("alwaysMe"), 
        String::from("alme@y.com"),
        String::from("thing"),
        false
    );
    println!("{} | {} | {} | {}", profile.0, profile.1, profile.2, profile.3);
}
