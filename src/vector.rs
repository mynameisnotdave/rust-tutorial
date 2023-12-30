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
}