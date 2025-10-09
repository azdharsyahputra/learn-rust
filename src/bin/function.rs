fn main() {
    mulai_program();
}

fn mulai_program() {
    halo();
    jumlahkan(4, 7);
}

fn halo() {
    println!("Selamat datang!");
}

fn jumlahkan(a: i32, b: i32) {
    let hasil = a + b;
    println!("Hasil penjumlahan: {}", hasil);
}
