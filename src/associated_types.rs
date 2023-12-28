use std::fmt::format;

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