struct User {
    name: String,
    age: u32,
    email: String,
}

fn constract_user(name: String, age: u32, email: String) -> User {
    User {
        name,
        age,
        email,
    }
}

fn main() {
    let user = constract_user()
}