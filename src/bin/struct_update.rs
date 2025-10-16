fn main() {
    let user1 = User {
        username: String::from("Ajar"),
        email: String::from("ajar@rustmail.com"),
        active: true,
        login_count: 1,
    };

    let user2 = User {
        email: String::from("ajar_new@mail.com"),
        ..user1
    };

    println!("User2 email: {}", user2.email);
}

struct User {
    username: String,
    email: String,
    active: bool,
    login_count: u32,
}
