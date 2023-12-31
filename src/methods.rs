#[derive (Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// An implementation is the implementation of methods for a given type.
impl Rectangle {
    fn area(&self) -> u32 {
        // When Rectangle is defined elsewhere as a variable, these 'self' sections
        // get replaced by the name of the variable that is referencing the struct.
        return self.width * self.height
    }
}

// It is possible to have more than one impl block.
impl Rectangle {
    // This is an associated function. It is associated because it does not have self
    // as its first parameter.
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }
}

// Here, an associated function is called C++ style and with its defined parameters.
// It can be used in a way that instantiates a struct.
fn associated_type_init() -> () {
    let rec1: Rectangle = Rectangle::new(5, 10);
}

/* Some key semantics here in Rust:
        A method always has a return type, their first parameter is always self,
        and that self refers to the struct or enum that the method is being called on.

        A function is just a plain old 'fn' function and can be associated or not
        associated with anything.
*/

struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(self: &Self) {
        println!("the current state is {}", self.color);
    }

    // One can use &mut self when a struct method mutates the value of one of its members.
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}

enum TrafficLightColour {
    Red,
    Yellow,
    Green
}

// In this impl block, match is used in the impl's method to match the enum.
// Note the use of match 'self' and how 'Self::' is used.
impl TrafficLightColour {
    fn colour(&self) -> &str {
        match self {
            Self::Yellow => "yellow",
            Self::Red => "red",
            Self::Green => "green"
        }
    }
}