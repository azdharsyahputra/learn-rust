use std::fmt::Display;

struct Kotak<T>{
    isi: T,
}

impl <T: Display> Kotak<T> {
    fn new(value: T) -> Self{
        Kotak {isi : value}
    }

    fn buka(&self){
        println!("Isi Kotaknya: {}", self.isi);
    }    
}

fn main(){
    let kotak_angka = Kotak::new("123");
    let kotak_text = Kotak::new("Azdhar");

    kotak_angka.buka();
    kotak_text.buka();
}