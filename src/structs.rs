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
        // the name(s) of the function parameter(s) and the name(s) of the member(s) match(es).
        username,
        email,
        sign_in_count: 1
    }
}


// These are tuple structs.
struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() -> () {
    let black = Colour(0,0,0);
    let origin = Point(0,0,0);
}

fn tuple_struct_impl() -> () {
    let v: Point = Point(0, 127, 255);
    check_color(v);

}

fn check_color(p: Point) {
    // This function demonstrates how to destructure a tuple struct.
    // Additionally, it shows that an underscore may be used if there is no reason for a
    // value in the tuple struct to be reassigned (if it were to be accessed by index).

    // Also, the types of a destructured tuple struct cannot be individually defined.
    // The type must remain as Point.
    let Point(x, _, z): Point = p;
    assert_eq!(x, 0);
    assert_eq!(z, 255);

}

struct Person {
    name: String,
    age: u8,
    hobby: String
}

fn assign_struct_val_from_var() -> () {
    // Here one can see that it is also possible to implicitly assign a value 
    // to a member of an instantiated struct if there is a scoped variable with the same name
    // as the name of the struct member.
    let age: u8 = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding")
    };
}

// This shows how to use an attribute to get a struct to print.
// Additionally, it shows how to use the dbg macro.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn print_struct() -> () {
    let scale: u32 = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    dbg!(&rect1);

    println!("{:?}", rect1);
}