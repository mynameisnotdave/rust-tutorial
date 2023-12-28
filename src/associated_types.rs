trait MyTrait {
    // Inside of traits we can define associated types.
    type MyType;

    fn get_my_type(&self) -> Self::MyType;
}

struct MyStruct {}

impl MyTrait for MyStruct {
    // The type inside that trait must be declared inside its implementations.
    type MyType = i32;

    fn get_my_type(&self) -> Self::MyType {
        return 42;
    }
}

trait Foo {
    fn method(&self) -> String;
}

// It is possible to implement traits on primitive types.
impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8:{}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

// Static dispatch is done by implementing the expected types that the function will take.
// The trait's methods can then work properly.
fn static_dispatch<T: Foo>(a: T) {
    a.method();
}

fn static_dispatch_u8<T: Foo>(a: u8) {
    a.method();
}

// Dynamic dispatch is done by the dyn type which will find out which impl to call at
// runtime.
fn dynamic_dispatch(a: &dyn Foo) {
    a.method();
}

fn misc() -> () {
    let x: u8 = 5;
    let y: String = "Hello".to_string();
    static_dispatch(x);
    dynamic_dispatch(&y);
}

// What all of this is doing here is dynamic dispatch through and through.
// I'm basically allocating anything and everything associated with this
// trait to a heap of memory, where all of its associated items can be managed.
trait MyTrait2 {
    fn f(&self) -> Box<dyn MyTrait2>;
}

impl MyTrait2 for u32 {
    fn f(&self) -> Box<dyn MyTrait2 > { Box::new(42) }
}

impl MyTrait2 for String {
    fn f(&self) -> Box<dyn MyTrait2> { Box::new(self.clone()) }
}

fn my_function(x: Box<dyn MyTrait2>) -> Box<dyn MyTrait2> {
    x.f()
}

fn misc2() -> () {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));
}