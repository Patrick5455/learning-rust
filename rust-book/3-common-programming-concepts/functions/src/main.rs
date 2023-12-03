//Functions
/***
Rust code uses snake case as the conventional style for function and variable names,
 in which all letters are lowercase and underscores separate words.

 Parameters
We can define functions to have parameters, which are special variables that are part of a function’s signature.
When a function has parameters, you can provide it with concrete values for those parameters.
Technically, the concrete values are called arguments, but in casual conversation,
people tend to use the words parameter and argument interchangeably
for either the variables in a function’s definition or the concrete values passed in when you call a function.
 */
fn another_function(x: i32, unit_label: char) {
    println!("The measurement is {x} {unit_label}");
}

/***
In Rust statements end with a `;` hence do not return a value hence we can't do x = y = 6 or x = y = 5+1
Expressions on the other end ends without a ';' and they return a value.
This is why return values in Rust have no `;`
 */

/***
Functions can return values to the code that calls them.
We don’t name return values, but we must declare their type after an arrow (->).
In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
You can return early from a function by using the return keyword and specifying a value,
but most functions return the last expression implicitly.
Here’s an example of a function that returns a value:
 */

fn five() -> i32 {
    5
}

fn plus_one (x: i32) -> i32 {
    x+1
}

fn main() {
    another_function(54, 'p');
    let  x = five();
    println!("The value of x is {x}");
    //let mut yi32 =  x = plus_one(x); // This would result in a Type mismatch error, "expected `i32` but found `()`"
   // println!("The value of x+1 is {}", y); //throws error when we try to print y except we use a format as (:?) and y uses type `y: ()`
    println!("The value of x is {}", plus_one(x));
}
