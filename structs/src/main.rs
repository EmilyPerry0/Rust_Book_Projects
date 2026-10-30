struct User {
    active: bool,
    name: String,
    email: String
}

struct UnitStruct;
struct Point(i32,i32,i32);

fn main() {
    let first_user = User {
        active: true,
        name: String::from("Name"),
        email: String::from("a@example.com")
    };
    println!("{}", first_user.name);
    build_user(String::from("Test_Name"), String::from("TEST_EMAIL"));
    let second_user = User {
        active: false,
        ..first_user
    };
    println!("{}", second_user.name);
    let un = UnitStruct;
    let coordinate = Point(1,3,5);
    println!("{}", coordinate.1);
}

fn build_user(name: String, email: String) {
    let new_user = User {
        active: true,
        name,
        email
    };
    println!("{} {} {}", new_user.active, new_user.name, new_user.email);
}