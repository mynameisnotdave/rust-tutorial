/// Traits are Rust's version of interfaces.
trait Animal {
    fn sound(&self) -> String;
}

struct Sheep;
struct Cow;
struct Dog;

// The trait is then implemented for different structs (or enums).
impl Animal for Sheep {
    fn sound(&self) -> String {
        String::from("Maah")
    }
}

impl Animal for Cow {
    fn sound(&self) -> String {
        String::from("Mooh")
    }
}

trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}
// Methods in the trait that have no content must be implemented in their corresponding
// implementation block.
impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("I'm a good student")
    }
}

struct Teacher {}
impl Hello for Teacher {
    fn say_something(&self) -> String {
        String::from("I'm a good teacher")
    }
}

fn do_stuff_with_the_traits() -> () {
    let s = Student {};
    // This can be called straight from instantiating the struct, as there is an
    // implementation for Student, which inherits the traits of Hello.
    assert_eq!(s.say_hi(), "Hi");

}