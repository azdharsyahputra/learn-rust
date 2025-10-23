fn main() {
    let mut vegetables = vec!["Tomat", "Wortel", "Cabai"];

    for v in &mut vegetables {
        *v = match *v {
            "Tomat" => "Tomat Segar",
            "Cabai" => "Cabai Merah",
            other => other,
        };
    }

    println!("Setelah update: {:?}", vegetables);
}