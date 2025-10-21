enum Gadget{
    Phone(String, i32),
    Laptop(String),
    Console,
}

impl Gadget {
    fn describe(&self){
        match self {
            Gadget::Phone(brand, year) => {
                println!("Handphone Merk {}, Rilisan tahun {}",brand, year);
            }
            Gadget::Laptop(brand) => {
                println!("laptop Merk {}",brand);
            }Gadget::Console => {
                println!("Console Adalah gadget untuk gaming");
            }
        }
    }

    fn is_portable(&self) -> bool{
        match self {
            Gadget::Phone(_,_) | Gadget::Laptop(_) => true, Gadget::Console => false
        }
    }
}
fn main(){
    let hp = Gadget::Phone(String::from("Xiaomi"), 2025);
    let laptop = Gadget::Laptop(String::from("HP Victus 15"));
    let console = Gadget::Console;

    hp.describe();
    laptop.describe();
    console.describe();

    println!("Is Phone Portable? {}", hp.is_portable());
    println!("Is Laptop Portable? {}", laptop.is_portable());
    println!("What about console ? {}", console.is_portable());

}