trait Greet{
    fn greet(&self);
}

struct Human{
    name: String
}

struct Robot{
    id: i32
}

impl Greet for Human{
    fn greet(&self) {
        println!("Halo Aku {}", self.name)
    }
}

impl Greet for Robot {
    fn greet(&self) {
        println!("Bip Bip Bop {}", self.id)
    }
}

fn main(){
    let manusia = Human {name: "Diana".into()};
    let robot = Robot{id: 39.into()};
    
    let mut entities : Vec<Box<dyn Greet>> = Vec::new();
    entities.push(Box::new(manusia));
    entities.push(Box::new(robot));

    for e in entities{
        e.greet();
    }
}