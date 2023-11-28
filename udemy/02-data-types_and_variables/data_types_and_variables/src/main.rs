use std:: mem; // import the memory library from the standard library
fn main() {
    println!("Data types and variables!");
    let a: u8 =123; // unsigned 8-bit integer 0-255
    //let binding makes a variable immutable by default
    println!("a = {}", a); // immutable variable

    //u = unsigned, 0 to 2^n - 1, where n is the number of bits, e.g. u8 = 0 to 2^8 - 1 = 0 to 255
    //i = signed, -2^(n-1) to 2^(n-1) - 1, e.g. i8 = -2^(8-1) to 2^(8-1) - 1 = -128 to 127
    let mut b: i8 = 0; // mutable variable
    println!("b= {}", b);

    //Type inference:
    let c= 123456789; // i32 - rust compiler infers the type, c is immutable
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    //mem::size_of_val(&c) returns the size of the variable in bytes

    //Data types:
    //u8, u16, u32, u64,
    // i8, i16, i32, i64,
    // f32, f64,
    // char, bool,
    // isize, usize (isize and usize depend on the computer architecture)
    //isize is the pointer size, usize is the unsigned pointer size
    // usize = unsigned size, isize = signed size



}
