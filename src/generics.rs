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



impl<T, U> TwoTypedPoint<T, U> {
    /// The return generic types in this method are T and W because it returns a
    /// TwoTypedPoint of 'self.x' (a TwoTypedPoint that is asking for T and U, and
    /// x in TwoTypedPoint = T), and then an an 'other' that is asking for a TwoTypedPoint
    /// of V and W, W according itself to y.
    /// Additionally, the purpose of the method is to mixup the values of two previously
    /// assigned structs.
    fn mixup<V, W>(self, other: TwoTypedPoint<V, W>) -> TwoTypedPoint<T, W> {
        TwoTypedPoint {
            x: self.x,
            y: other.y
        }
    } 
}

fn two_typed_point() -> () {
    // Generic structs can also take multiple different defined types.
    let p: TwoTypedPoint<i32, String> = TwoTypedPoint{x: 5, y: "hello".to_string()};

    let p1: TwoTypedPoint<i32, i32> = TwoTypedPoint {x: 5, y: 10};
    let p2: TwoTypedPoint<&str, char> = TwoTypedPoint {x: "Hello", y: 'Д'};
    let p3: TwoTypedPoint<i32, char> = p1.mixup(p2);
}