fn main(){
ownership();
ownership_movement();
clone();
if_expression();
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
fn clone(){
    let name1 = String::from("Ajar");
    let name2 = name1.clone();

    println!("{} {}",name1, name2);
}

fn if_expression(){
    let a = 4;
    let result: &str = if a >= 8{
        "OKEII"
    }else if a  >=5 {
        "SOSO"
    }else if a >=3 {
        "BAD"
    }else{
        "VERY BAD"
    };
    println!("{}",result);
}