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