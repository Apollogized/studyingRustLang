#[derive(Debug)]

enum Gender {
    Male, Female
}


fn main() {
    
    let player_male: Gender = Gender::Male;
    let player_female: Gender = Gender::Female;

    println!(": {:?} and {:?}", player_female, player_male);
} 
