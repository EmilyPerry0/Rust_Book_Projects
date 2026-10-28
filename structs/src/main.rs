struct User {
    active: bool,
    name: String,
    email: String
}

fn main() {
    let mut first_user = User {
        active: true,
        name: String::from("Name"),
        email: String::from("a@example.com")
    };
    println!("{}", first_user.name);
    build_user(String::from("Test_Name"), String::from("TEST_EMAIL"));
}

fn build_user(name: String, email: String) {
    let new_user = User {
        active: true,
        name,
        email
    };
    println!("{} {} {}", new_user.active, new_user.name, new_user.email);
}