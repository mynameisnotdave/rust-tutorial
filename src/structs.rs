// Here we are defining the members of the struct and their types.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// Then, we are assigning values to the struct members.
fn struct_impl() -> () {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };
}

// Here, we have a function that returns the User struct.
fn build_user(email: String, username: String) -> User {
    return User {
        active: true,
        // Declaring the assignment of values to the two members below is not necessary if
        // the name(s) of the function parameter(s) and the name(s) of the member(s) match.
        username,
        email,
        sign_in_count: 1
    }
}