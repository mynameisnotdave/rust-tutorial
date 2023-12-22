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
    let user2 = User {
        email: String::from("another@example.com"),
        // If one value from the previous assignment just needs to be changed then
        // define that change of value and then just use the following syntax to declare
        // that 'everything else should be kept the same as before'
        ..user1
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