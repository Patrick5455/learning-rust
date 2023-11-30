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
    A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    We create a tuple by writing a comma-separated list of values inside parentheses.
    Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.
     */

    let tup:(i8, &str, bool, f32) = (1, "string", false, 6.4);
    println!("tuple value at index 1: {}", tup.1);

    /***
    The variable tup binds to the entire tuple because a tuple is considered a single compound element.
    To get the individual values out of a tuple,
    we can use pattern matching to destructure a tuple value, like this:
     */

    let (x, y, z, a) = tup;
    println!("x:{x}, y:{y}, z:{z}, a:{a}");
    //This is called destructuring because it breaks the single tuple into four parts.

    /***
    We can also access a tuple element directly by using a period (.)
    followed by the index of the value we want to access. For example:
     */

    let abc:(bool, &str, f32, i32) = (true, "down-to-earth", 7.0, 7);
    let a_bool = abc.0;
    let a_str = abc.1;
    let a_float = abc.2;
    let a_int = abc.3;

    println!("a_bool: {a_bool}, a_str: {a_str}, a_float:{a_float}, a_int:{a_int}");

    /***
    The tuple without any values has a special name, unit.
    This value and its corresponding type are both written () and represent an empty value or an empty return type.
    Expressions implicitly return the unit value if they don’t return any other value.
     */

    //The Array Type
    /***
    Another way to have a collection of multiple values is with an array.
    Unlike a tuple, every element of an array must have the same type.
    Unlike arrays in some other languages, arrays in Rust have a fixed length.
     */
     let a = [7, 8, 9, 10];
    let b:[&str;2] = ["abc", "def"];

    /***
    Arrays are useful when you want your data allocated on the stack rather than the heap
    or when you want to ensure you always have a fixed number of elements.

    An array isn’t as flexible as the vector type, though.
    A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.
    If you’re unsure whether to use an array or a vector, chances are you should use a vector.

    However, arrays are more useful when you know the number of elements will not need to change.
    For example, if you were using the names of the month in a program,
    you would probably use an array rather than a vector because you know it will always contain 12 elements:


    You write an array’s type using square brackets with the type of each element, a semicolon,
    and then the number of elements in the array, like so:
     */

    let months:[&str;12] = [
        "January", "February", "March", "April",
        "May", "June", "July", "August",
        "Spetember", "October", "November", "December"
    ];

    println!("month 5: {}", months[5]);

    /***
    You can also initialize an array to contain the same value for each element
    by specifying the initial value, followed by a semicolon,
    and then the length of the array in square brackets, as shown here:
     */

    let a = [3;5];
    let b:[bool;5]=[false;5];
    let c:[&str;10]=["hello";10];

    /***
    The array named a will contain 5 elements that will all be set to the value 3 initially.
    This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.
     */

    println!("repeating value in array a: {}", a[3]);
    println!("repeating value as array b: {}", b[3]);
    println!("repeating value as array c: {}", c[9]);




}
