// Constants 
// In the last lesson, we have studied variables and mutability.
// to create a new cargo directory within the current, do cargo init.
 
fn main() {
    println!("Hello, world!");
    let mut x = 5;
    // const mut y = 10; 
    const Y: i32 = 10; 
    println!("The value of x is: {}", x);
    println!("The value of Y is: {}", Y);
    println!("The value of PI is: {}", PI);
    println!("The value of 3 Hours in seconds is: {}", THREE_HOURS_IN_SECONDS);
}
// You can declare a constant with a type annotation
const PI: f64 = 3.141592653;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// Like immutable variables that we have discussed the last part, constants actually are values are bound to a name and are not allowed to change, exactly like variables.
// But in fact, there are a few differences between constantsa and variables, that's why I was very careful the last time, I told you do no confused the variables with the constants.
// Granted both are immutable but the first difference between the constant and a variable that in a constant, you're not allowed to use the keyword 'mut' with constant

// you declare a constant by using the keyword 'const' and suggests that it should be Capital, and Type annotation should exist and then the assignment for the variable.

// A constant by default is immutable, not only that, but also you're unable to change it to mutable.
// It will always stay immutable and you can never change a constant to mutable.
// You need to provide a type for the constant. Because you are obliged to enter or to add the Type Annotation like i32 for instance.


// Also a distinctive feature of const is that you can declare const in any scope, including the global scope.
// Global scope means outside the main function.

// So that's it, those are the rules of constants in rust.
// In the next video we are going to discuss Shadowing and explain what shadowing is in Rust.