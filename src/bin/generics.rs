use std::fmt::Display;

fn lebih_besar<T>(a: T, b: T)
where T: PartialOrd + Display,
{
    if a > b{
        println!("{} Lebih besar dari {}", a, b);
    }else if a < b {
        println!("{} Lebih kecil dari {}", a, b);
    }else {
        println!("{} Sama Dengan {}", a, b);
    }
}

fn main(){
    lebih_besar(20, 25);
    lebih_besar(1, 1);
    lebih_besar(2, 1);
    lebih_besar("Rust", "Php");
}