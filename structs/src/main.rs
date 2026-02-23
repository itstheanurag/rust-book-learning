#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = build_user(
        String::from("gaurav@example.com"),
        String::from("gauravanurag"),
    );

    println!("{:#?}", user1.active);
    println!("{:#?}", user1.username);
    println!("{:#?}", user1.email);
    println!("{:#?}", user1.sign_in_count);
}

// this function builds a new user
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
