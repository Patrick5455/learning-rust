# learning-rust
repo to document my codes while learning rus  t

## 02 - Data Types and Variables

- Rust is a statically typed language, which means that it must know the types of all variables at compile time
- The compiler can usually infer what type we want to use based on the value and how we use it
- If many types are possible, we must add a type annotation
- Rust is a block-scoped language
- Variables are immutable by default
- We can make variables mutable by adding the `mut` keyword
- Constants are always immutable
- Constants must be annotated
- Constants can only be set to a constant expression, not the result of a function call or any other value that could only be computed at runtime
- Shadowing allows us to reuse the variable name
- Shadowing is different from marking a variable as `mut`, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword
- Shadowing is useful when we want to change the type of a variable but reuse the same name
- Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters
- Rust has two primitive compound types: tuples and arrays
- Tuples have a fixed length: once declared, they cannot grow or shrink in size
- Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same
- Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples
- Arrays are useful when you want your data allocated on the stack rather than the heap