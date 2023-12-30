use std::any::Any;

fn vecs() -> () {
    let arr: [u8; 3] = [1, 2, 3];
    // Use Vec::from() to convert an array into a vector.
    let v: Vec<u8> = Vec::from(arr);

    // vec![] and vec!() are the same thing
    let v: Vec<u8> = vec![1, 2, 3];
    is_vec(v);

    let v: Vec<u8> = vec!(1, 2, 3);
    is_vec(v.clone());

    // Using vec! with an array will create a halfway sort of type that is Vec<[u8; 3]>,
    // where the Vec's type is of an array.
    let mut v1: Vec<u8> = Vec::new();
    is_vec(v1.clone());

    for i in &v {
        v1.push(*i);
    }

    assert_eq!(v, v1);

    println!("Success!");
}

fn is_vec(v: Vec<u8>) {}

fn extend_vector() -> () {
    let mut v1: Vec<i32> = Vec::from([1, 2, 4]);
    v1.pop(); // This removes the last element (4)
    v1.push(3); // This adds 3 to end of the vector

    // This is how to 'map' a new vector from an old one, by using the extend function.
    // The extend function's parameter should be a borrowed version of the old vector.
    let mut v2: Vec<i32> = Vec::new();
    v2.extend(&v1);

    assert_eq!(v1, v2);

    println!("Success!");
}

fn string_to_vec() -> () {
    // str to Vec
    let s: &str = "hello";
    let v2: Vec<u8> = Vec::from(s);

    // String to Vec
    let t: String = String::from("hello");
    let v3: Vec<u8> = t.into_bytes();    
    assert_eq!(v2, v3);
}

fn index_vec() -> () {
    // Using the get method below forces us to use usize as the Vector's type (for
    // numerical elements).
    let mut v: Vec<usize> = Vec::from([1,2,3]);
    for i in 0..5 {
        // Don't use v[i] here. Use v.get, which returns an option type. This can safely
        // handle out of bounds situations by returning a Some() type if there is something
        // that can be accessed, and then if there is something out of bounds then it's None.
        println!("{:?}", v.get(i))
    }
    for i in 0..5 {
        match v.get(i) {
            // What does this do here?
            // If the value is indexable, iterate it up one more up to index five.
            Some(e) => v[i] = e + 1,
            // If it doesn't exist, push the unindexable value and add two.
            None => v.push(i + 2)
        }
    }
    assert_eq!(v, vec![2,3,4,5,6]);

    println!("Success!");
}

/* 
-----------------VEC AND SLICE DIFFERENCE----------------
                    Vec<T> = mutable
                    slice = read-only
-------------------------------------------------------
*/

fn vecs_and_slices() -> () {
    let mut v: Vec<i32> = vec![1, 2, 3];

    // Slice of Vec
    let slice1: &[i32] = &v[..];

    let slice2: &[i32] = &v[0..v.len()];

    assert_eq!(slice1, slice2);

    // Slices are read-only
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3: &[i32] = &v[0..];

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!");
}

fn vec_capacity() -> () {
    // Box<dyn Any>> to instantiate a generic type of an unknown value.
    // Use ::with_capacity() to instantiate a Vec with a predefined capacity.
    let mut _vec_unknown: Vec<Box<dyn Any>> = Vec::<Box<dyn Any>>::with_capacity(10);

    let mut vec: Vec<i32> = Vec::<i32>::with_capacity(10);
    // capacity != len. The Vec is empty so len should be 0.
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    for i in 0..10 {
        vec.push(i);
    }

    // I have now pushed ten elements to the Vec so len should now equal capacity.
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    // The following may trigger a reallocation of the Vec. The compiler will take over here
    // and double the capacity, even if I have added just one more element.
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);


}


// Vectors cannot store elements of different types on their own.
// Here I will use enums and trait objects to allow the Vector to store elements
// of different types.
#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String)
}

fn vec_to_store_different_types_enum() -> () {
    // How to allocate a value to an element inside an enum inside a Vector.
    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];
}

// Now I will use trait objects to store differing elements inside a Vector.
trait IpAddrTrait {
    fn display(&self);
}

struct V4(String);
impl IpAddrTrait for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}

struct V6(String);
impl IpAddrTrait for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

fn vec_to_store_different_types_trait() -> () {
    // Trait objects must be boxed, and so must the entities that the trait applies itself to.
    let v: Vec<Box<dyn IpAddrTrait>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string()))
    ];

    for ip in v {
        ip.display();
    }
}
