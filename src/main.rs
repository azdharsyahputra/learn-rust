const AJAR: i32 = 10;

fn main(){
    println!("{}",AJAR);
    variable_scope();
    stack_heap();
    string();
    string_type();
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