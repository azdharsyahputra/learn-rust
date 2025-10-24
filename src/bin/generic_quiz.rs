use std::fmt::Display;

fn tukar<T: Display>(a: T, b: T) -> (T, T) {
    println!("Menukar posisi: {} <-> {}", a, b);
    (b, a)
}

fn main() {
    let hasil = tukar(10, 20);
    println!("{:?}", hasil);

    let hasil2 = tukar("Rust", "PHP");
    println!("{:?}", hasil2);
}
