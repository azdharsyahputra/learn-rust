fn main(){
 combine();
}

fn combine(){
 hello();
 pernjumlahan(3, 5);
}

fn hello(){
    println!("Hai Ajarrr");
}
fn pernjumlahan(a: i32, b: i32){
    let hasil = a + b;
    println!("Hasil Penjumlahan adalah {}",hasil);
}