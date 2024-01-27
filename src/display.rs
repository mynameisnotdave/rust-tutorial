use std::fmt;

struct Point {
    x: i32,
    y: i32
}

// This is how one can manually derive the Display trait for their custom type to
// print the type's properties out.
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // No semicolon here as this function is not void.
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn point_to_string() -> () {
    let origin: Point = Point { x: 0, y: 0 };

    assert_eq!(origin.to_string(), "The point is (0,0)");
    assert_eq!(format!("{}", origin), "The point is (0,0)");

    println!("Success!");
}