use std::io;

fn main(){
    println!("Siapa Namamu?");

    println!("Nama saya ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Siapa ya?");

    println!("Namamu adalah {guess}");
}