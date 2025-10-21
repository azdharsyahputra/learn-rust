enum Direction{
    Up,
    Down,
    Left,
    Right,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main(){
    let movement = Direction::Left;

    match movement {
        Direction::Up => println!("Bergerak Keatas"),
        Direction::Down => println!("Bergerak Kebawah"),
        Direction::Left => println!("Bergerak Kiri"),
        Direction::Right => println!("Bergerak Kanan"),
    }

    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Program berhenti"),
        Message::Move { x, y } => println!("Pindah ke posisi ({}, {})", x, y),
        Message::Write(text) => println!("Menulis pesan: {}", text),
        Message::ChangeColor(r, g, b) => println!("Ubah warna jadi RGB({}, {}, {})", r, g, b),
    }
}
