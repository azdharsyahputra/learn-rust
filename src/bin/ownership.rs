fn main(){
ownership();
ownership_movement();
}

fn ownership(){
    let a: i32 = 20;
        {
            let b: i32 = 30;
            println!("{}",b);
        }

    println!("{}",a);
}

fn ownership_movement(){
    let name1 = String::from("Azdhar");
    
    let name2 = name1;

    println!("{}",name2);
}