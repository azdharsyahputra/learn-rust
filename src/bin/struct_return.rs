fn build_user(username: String, email: String) -> User{
    User{
        username,
        email,
        active: true,
        login_count: 0,
    }
}
fn main(){
    let user2 = build_user(String::from("rustacean"), String::from("rustacean@rust"));
    println!("User: {}", user2.username);
    println!("Email: {}", user2.email);
    println!("Active: {}", user2.active);
}

struct User{
    username: String,
    email: String,
    active: bool,
    login_count: u32,
}