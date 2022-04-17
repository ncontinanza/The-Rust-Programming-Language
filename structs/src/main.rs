struct User {
    active: bool,
    username: String,  // We use String instead of &str because we want each instance of this struct to own
    email: String, // all of its data, and for that data to be valid for as long as the entire struct is valid
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let origin = Point(0, 0, 0);
    println!("y coordinate: {}", origin.1);

    let mut user1 = User {
        active: true,
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 1,
    };

    user1.email = String::from("anothermail@example.com"); // The whole instance must be mutable!
    println!("user1 username is: {}", user1.username);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // struct update syntax -> used to easily reuse another instance's fields.
    };
    // But we can no longer use user1 from here on, as it was moved.

}

fn build_user(email: String, username: String) -> User {
    User {
        email,  // Field init shorthand
        username, // Field init shorthand
        active: true,
        sign_in_count: 1,
    }
}