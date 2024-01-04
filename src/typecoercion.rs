/* The video explains that 1000 as u8 would equal a value of 232. This is because u8 accepts
    a maximum value of 256, and type coercion in Rust means that the compiler will force the
    defined value to adhere to the rules of the type that it is forced into, IF the allow overflow attribute is defined. 
    So 1000 - 256 = 744, still too big. 744-256=488, still too big. 488-256 = 232, this is okay as u8.

    -1i8 as u8 would make -1 = 255, as it will turn the value on its head.
*/

fn coercion() -> () {
    let decimal: f32 = 97.123_f32;

    let integer: u8 = decimal as u8; // will make 97.123 into 97.

    let c1: char = decimal as u8 as char;
    let c2 = integer as char; // 97 = a

    assert_eq!(integer + 1, 'b' as u8);

    println!("Success!");
}

#[allow(overflowing_literals)] // required to allow overflow
fn allow_overflow() -> () {
    let v: u8 = 1000 as u8;
}