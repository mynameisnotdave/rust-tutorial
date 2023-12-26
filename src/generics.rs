struct A; // concrete type
struct S(A); // still a concrete type
struct SGen<T>(T); // generic type

fn reg_fn(_s: S) {} // function that uses a concrete type as a parameter

fn generic<T>(_s: SGen<T>) {} // this is a generic function

// The type is implementing a trait.
fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a+b
}

fn use_generic() -> () {
    // An example of how to use function arguments to implicitly provide a type, if the
    // parameters are also T.
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(11.1, sum(5.5, 5.6))
}

struct Point<T> {
    x: T,
    y: T
}

fn assign_val_to_point_struct() -> () {
    // This is how to instantiate a generic struct. 
    let x: Point<i32> = Point{x: 4, y: 8};
}

// 'U' is the name of the second type as it is the letter after T.
struct TwoTypedPoint<T, U> {
    x: T,
    y: U
}

fn two_typed_point() -> () {
    // Generic structs can also take multiple different defined types.
    let p: TwoTypedPoint<i32, String> = TwoTypedPoint{x: 5, y: "hello".to_string()};
}