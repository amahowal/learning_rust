// These are tuple structs, fields have types but not names
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// This is a unit struct, no data, but can have traits
struct AlwaysEqual;

// Structs need to user wholely owned type rather than
// a reference like a string slice so that the data is owned/valid
// as long as the struct is. Lifetimes can be used also... later
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        // any remaining fields get their value from user1
        ..user1
    };
    // GOTCHA: depending on the types in user1 that are MOVED to
    //         user2, user1 may or may not be valid after this op
}
