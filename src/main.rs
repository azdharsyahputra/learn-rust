const AJAR: i32 = 10;

fn main(){
    println!("{}",AJAR);
    variable_scope();
    stack_heap();
    string();
    string_type();
    recall();
}

fn variable_scope(){
    let ajar = 1;

    {
        let raja = 2;

        println!("inner scope {}",raja);
    }

    println!("inner scope {}",ajar);
}

fn stack_heap(){
    function_a();
    function_b();
}

fn function_a(){
    let a = 10;
    let b = String::from("ajar");

    println!("{}{}", a,b);
}
fn function_b(){
    let a = 10;
    let b = String::from("aja");
    println!("{}{}", a,b);
}
fn string(){
    let name: &str = " Ajar ";
    let trim: &str = name.trim();

    println!("{}",trim)
}

fn string_type(){
    let mut name: String = String::from("Ajar");

    println!("{}", name);

    name.push_str(" Syahputra");
    println!("{}",name);

    let diana = name.replace("Ajar", "Diana");
    println!("{}",diana);
}
fn recall(){
    recall_a();
    recall_b();
    potong_paras();
}
fn recall_b(){
    let a: &str = " Farras ";
    let trim: &str = a.trim();

    println!("{}",trim);
}
fn recall_a(){
    let a: &str = " pacar citra ";
    let trim: &str = a.trim();

    println!("{}",trim);
}

fn potong_paras(){
    let  mut name: String = String::from("Faras");
    println!("{}", name);

    name.push_str("subhan");
    println!("{}",name);

    let panjang: String = name.replace("Faras", "Citra");
    println!("{}",panjang);
}

fn ownership(){
    let a = 10;
    {
        let b = 10;
        println!("{}",b);
    }

    println!("{}",a);

}
