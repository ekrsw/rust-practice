struct User {
    name: String,
    email: String,
}

fn constract_user(name: String, email: String) -> User {
    User {
        name,
        email,
    }
}

fn main() {
    let user = constract_user(
        String::from("Eisuke"),
        String::from("example@example.com")
    );
    println!("name: {}, email: {}", user.name, user.email);

    let user2 = User {
        name: String::from("Koresawa"),
        ..user
    };
    println!("name: {}, email: {}", user2.name, user2.email);
}