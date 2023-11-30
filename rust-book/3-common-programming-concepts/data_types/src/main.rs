fn main() {

    /***
    Every value in Rust is of a certain data type,
    which tells Rust what kind of data is being specified so it knows how to work with that data.
    We’ll look at two data type subsets: scalar and compound.
     */

    //Scalar Types

    /***
    A scalar type represents a single value.
    Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
     */

    //Integer Types
    /***
    An integer is a number without a fractional component.

    Each variant can be either signed or unsigned and has an explicit size.
    Signed and unsigned refer to whether it’s possible for the number to be negative—in other words,
    whether the number needs to have a sign with it (signed) or whether it will only ever be positive
    and can therefore be represented without a sign (unsigned). It’s like writing numbers on paper:
    when the sign matters, a number is shown with a plus sign or a minus sign;
    however, when it’s safe to assume the number is positive,
    it’s shown with no sign. Signed numbers are stored using two’s complement representation.

    Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive,
    where n is the number of bits that variant uses.
    So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127.
    Unsigned variants can store numbers from 0 to 2n - 1,
    so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.

    So how do you know which type of integer to use? If you’re unsure,
    Rust’s defaults are generally good places to start: integer types default to i32.
    The primary situation in which you’d use isize or usize is when indexing some sort of collection.


    Integer Overflow:
    Let’s say you have a variable of type u8 that can hold values between 0 and 255.
    If you try to change the variable to a value outside that range, such as 256,
    integer overflow will occur, which can result in one of two behaviors.
    When you’re compiling in debug mode, Rust includes checks for integer overflow
    that cause your program to panic at runtime if this behavior occurs.
    Rust uses the term panicking when a program exits with an error;
    we’ll discuss panics in more depth in the “Unrecoverable Errors with panic!”

    When you’re compiling in release mode with the --release flag,
    Rust does not include checks for integer overflow that cause panics.
    Instead, if overflow occurs, Rust performs two’s complement wrapping.
    In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold.
    In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on.
    The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have.
    Relying on integer overflow’s wrapping behavior is considered an error.

    To explicitly handle the possibility of overflow,
    you can use these families of methods provided by the standard library for primitive numeric types:
    Wrap in all modes with the wrapping_* methods, such as wrapping_add.
    Return the None value if there is overflow with the checked_* methods.
    Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
    Saturate at the value’s minimum or maximum values with the saturating_* methods.
     */


    //Floating-Point Types
    /***
    Rust also has two primitive types for floating-point numbers, which are numbers with decimal points.
    Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.
    The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision.
    All floating-point types are signed.
     */

    let x = 10.0; //f64
    let x: f32 = 10.9; //f32

    /***
    Floating-point numbers are represented according to the IEEE-754 standard.
    The f32 type is a single-precision float, and f64 has double precision.
     */

    //The Boolean Type

    let t = true;
    let f:bool = false;

    //The Character Type

    let c = 'c';
    let h:char = 'h';
    let heart_eyed_cat = '😻';

    /***
    Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes.
    Rust’s char type is four bytes in size and represents a Unicode Scalar Value,
    which means it can represent a lot more than just ASCII.
    Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.
    Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
    However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust.
     */


    //Compound Types
    /***
    Compound types can group multiple values into one type.
    Rust has two primitive compound types: tuples and arrays.
     */

    //The Tuple Type
    /***

     */

}
