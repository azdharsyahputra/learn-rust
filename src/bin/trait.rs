trait Speak {
    fn say_hello(&self);
}
struct Human;
struct Dog;


impl Speak for Human {
    fn say_hello(&self) {
        println!("Halo, aku manusia!");
    }
}

impl Speak for Dog {
    fn say_hello(&self) {
        println!("GOgogogogogo");
    }
}

fn greet<T: Speak>(thing: T){
    thing.say_hello();
}

fn main(){
    greet(Human);
    greet(Dog);
}