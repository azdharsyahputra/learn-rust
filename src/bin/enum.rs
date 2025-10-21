enum Transport {
    Car(String, i32),
    Bike(String),
    Train,
}

impl Transport {
    fn describe(&self){
        match self {
            Transport::Car(brand,year) => {
                println!("Mobil Merk {} tahun {}",brand, year);
            }
            Transport::Bike(brand) => {
                println!("Sepedeah Merk {}",brand);
            }
            Transport::Train =>{
                println!("Kereta api umum");
            }
        }
    }
    
    fn is_motorized(&self) -> bool {
        match self {
            Transport::Car(_,_ ) | Transport::Bike(_) => true, Transport::Train => false, 
        }
    }
}

fn main(){
    let car = Transport::Car(String::from("Toyota"), 1966);
    let bike= Transport::Bike (String::from("Polytron"));
    let train = Transport::Train;

    car.describe();
    bike.describe();
    train.describe();

    println!("Mobil Bermotor? {}", car.is_motorized());
    println!("Kereta Bermotor? {}", train.is_motorized());
}