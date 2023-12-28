use std::ops;

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

struct Foo;
struct Bar;


#[derive(PartialEq, Debug)]
struct FooBar;
#[derive(PartialEq, Debug)]
struct BarFoo;

// Overriding the trait so that it can take FooBar and not just numeric types.
impl ops::Add<Bar> for Foo {
    type Output = FooBar; // This is needed to declare the type of the output.

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

impl ops::Sub<Bar> for Foo {
    type Output = BarFoo;

    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}

fn add_sub_bar_foo() -> () {
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);
}

trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

// Traits can be function arguments. However, it must be as an impl.
fn summary(a: impl Summary) {
    let output: String = a.summarize();

    println!("{}", output);
}

/// Returning structs and having the return type as a trait object requires everything
/// to be boxed, the trait object must be boxed as dyn Animal, and the returned structs must
/// be boxed as well. Using dyn infers dynamic dispatch. Dynamic dispatch in this instance
/// is necessary because of the way this function is used, as it is assigned in a variable
/// in the function below and subsequently used to call the sound function that is
/// implemented in the trait. However, since this trait function is implemented for different
/// structs differently, then the decision of which function to call
/// must be declared at runtime.
///
/// Additionally, a box is an allocation of heap memory of whatever is inside the box.
/// This means I understand why everything has to be boxed here -- the sizes of what could be
/// returned from this function are not known, so it is not possible to store this sort of
/// thing on the stack.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn random_animal_impl() -> () {
    let random_number: f64 = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.sound());
}