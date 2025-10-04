use std::io;
fn main(){
    let a = [1,2,3,4,5];

    println!("choose one array {:?}",a);

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to load line");
    
    let index: usize = index
    .trim()
    .parse()
    .expect("index entered not numver");

    let element = a[index];

    println!("The entered value at index {index} is element {element}");
}