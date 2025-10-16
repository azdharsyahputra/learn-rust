struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // method pertama
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method kedua (mutable)
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // associated function (tanpa self)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let mut rect1 = Rectangle { width: 10, height: 5 };
    println!("Luas: {}", rect1.area());

    rect1.scale(2);
    println!("Setelah di-scale: {}x{}", rect1.width, rect1.height);

    let square = Rectangle::square(6);
    println!("Kotak: {}x{}", square.width, square.height);
}
