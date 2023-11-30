
//using a const in a global space
const FOUR_HOURS_IN_SECONDS : u32 = 60 * 60 * 4;

fn main() {

    // variables and mutability
    //let x:i32 = 5; //immutable
    let mut x:i32 = 5; //mutable with the `mut` keyword
    println!("x: {x}");
    x = 6;
    println!("x: {x}");

    //Constants
    /***
    First, you aren’t allowed to use mut with constants.
    Constants aren’t just immutable by default—they’re always immutable.
    You declare constants using the const keyword instead of the let keyword,
    and the type of the value must be annotated.
     */
    println!("global space const: {FOUR_HOURS_IN_SECONDS}");

    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;

    println!("const defined in main function: {THREE_HOURS_IN_SECONDS}");

    //Shadowing

    /***
    You can declare a new variable with the same name as a previous variable.
    Rustaceans say that the first variable is shadowed by the second,
    which means that the second variable is what the compiler will see when you use the name of the variable.
    In effect, the second variable overshadows the first,
    taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.
    We can shadow a variable by using the same variable’s name and repeating the use of the let keyword
     */

    let var = 2;
    println!("value of var in outer scope before overshadow is {var}");

    let var = 23 * var;

    {
        let var = var/4;
        println!("value of var in inner scope is {var}")
    }

    println!("value of var in outer scope (after overshadow) is {var}");

    /***
    The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again,
    we can change the type of the value but reuse the same name.
    For example, say our program asks a user to show how many spaces they want between some text by inputting space characters,
    and then we want to store that input as a number:
     */

    let spaces = "  ";
    let spaces = spaces.len();

    /***
    The first spaces variable is a string type and the second spaces variable is a number type.
    Shadowing thus spares us from having to come up with different names,
    such as spaces_str and spaces_num; instead, we can reuse the simpler spaces name.
    However, if we try to use mut for this, as shown here, we’ll get a compile-time error:
     */

    // let mut spaces: &str = "    ";   --> this code does not compile
    //  spaces = spaces.len();  --> The error says we’re not allowed to mutate a variable’s type:
                                     // e.g str to int as we have here



}
