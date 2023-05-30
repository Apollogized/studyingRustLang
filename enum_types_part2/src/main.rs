#[derive(Debug)]

enum CarModel {
    Fiat,
    Ford,
    Renault
}

fn car_nationality(car: CarModel){
    match car {
        CarModel::Fiat => println!("Italian car"),
        CarModel::Ford => println!("American car"),
        CarModel::Renault => println!("French car")
    }
}

fn main() {
    car_nationality(CarModel::Renault);
}
