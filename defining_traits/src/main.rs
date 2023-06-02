struct Person {
    name: String,
    age: i32

}

trait Voice {
    fn speak(&self);

    fn has_voice(&self) -> bool;
}

impl Voice for Person {
    fn speak(&self){
        println!("Hi, my name is {}", self.name);

    } 
    fn has_voice(&self) -> bool {
        if self.age > 1 {
            return true;      
        }
        return false;
    }

}

fn main() {
    let person = Person 
    {
        name: String::from("Myself"),
        age: 1
    };

    println!("{:?} \n", person.speak());
    println!("{} | {}", person.name, person.has_voice());
}
