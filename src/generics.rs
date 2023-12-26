struct A; // concrete type
struct S(A); // still a concrete type
struct SGen<T>(T); // generic type

fn reg_fn(_s: S) {} // function that uses a concrete type as a parameter

fn generic<T>(_s: SGen<T>) {} // this is a generic function