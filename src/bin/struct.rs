fn main(){
    let mut user1 = User{
        name: String::from("Ajar"),
        email: String::from("ajarganteng@hotmail.com"),
        active: true,
        login_count: 3,
    };

    println!("Username {}",user1.name);
    println!("Email {}",user1.email);
    println!("Status {}",user1.active);

    user1.login_count += 1;

    println!("Login Count {}",user1.login_count);
}
struct User{
    name: String,
    email: String,
    active: bool,
    login_count: u64,
}
