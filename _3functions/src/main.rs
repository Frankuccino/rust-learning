// Functions
// entry point
// any function / variable should be written in snake_case. Snake cases are letters in small and separated by an underscore.
// snake case: hello_world (Recommened - write functions & variables)
// kebab-case: hello-world
fn main() {
    hello_world();
    tell_height(175);
    human_id("Frank", 23, 175.5);
    let _x = {
        let price = 5;
        let qty = 10;
        price * qty
        // any expression that evaluates to a certain value, a certain mathematical operation will evaluate to the last line in that expression.
        // This is a very unique feature in Rust.
        // Alternatively you can do return
    };
    println!("Result is: {}", _x);
    let sum_values = add(5,10);
    println!("The sum result is {}", sum_values);
    println!("Value from function 'add is': {}", add(4, 6));

    // Calling the BMI function
    let weight: f64 = 70.0;
    let height: f64 = 1.75;
    let bmi  = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);
}

// Any function in Rust starts with the keyword fn followed by the name of your function, if we're not talking about the main function.
// If any other function you're going to write it should start with the reserved word in rust fn which stands for function. 

fn hello_world() {
    println!("Hello, Rust 🦀!")
}

// As you can notice, the function is written below the main function.
// This is indeed not available in all programming languages, you should define you function first, then call it afterwards (default knowledge about it).
// In programming this is called Hoisting, where you can actually define your function above or below then you can call it anywhere in your code.
// Hoisting also exists in JavaScript.

// We can also insert input values, 
// we can insert parameters and pass the arguments when we call the function.
fn tell_height(height: u32) {
    println!("My height is {}cm", height)
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old, and my height is {}cm", name, age, height);
} 

// So like other programming language, rust can return a value. Functions do not only print lines but they can actually return value, so we can do mathematical operations, and the result of that function is going to be whatever that mathematical operation will evaluate to.

// Expressions and Statements - This is a big deal in Rust, well in any other programming language but as we're talking about rust.
// Expression: Anything that returns a value.
// Statement: Anything that does not return a value.

// Expression 
// ----------
// 5
// true & false
// add(3,5)
// control flow (if {conditions}...)
// Expressions can also include blocks, the curly braces. ({code})

// So any variables outside of the main function should be declared with the const keyword 

// function returning values - let's say a function that adds two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Statement however almost always end with semicolon.
// Example: 
// 1. Variable declarations, like let x = 5,
// 2. Function definitions: fn foo() {}
// 3. Control flow statements: if condition { /* code */ } else { /*  code */ }, while condition { /* code */}, etc.

// Final Example
// BMI = height(kg)/height(m)^2

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
