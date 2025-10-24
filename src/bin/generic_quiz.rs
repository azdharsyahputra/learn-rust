use std::fmt::Display;

struct Pair<T>{
    first: T,
    second: T,
}

impl <T: Display> Pair<T>{
    fn new(value1: T, value2: T)-> Self{
        Pair{first: value1, second: value2}
    }

    fn normal(&self){
        println!("Sebelum Swap ({} {})", self.first, self.second);
    }

    fn swap(&self){
        println!("Hasil Swap ({} {})", self.second, self.first);
    }
}

fn main(){
    let swap1 = Pair::new(10, 20);
    let swap2 = Pair::new("Rust", "PHP");

    swap1.normal();
    swap1.swap();

    swap2.normal();
    swap2.swap();
}