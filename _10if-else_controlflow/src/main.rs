// Control Flow in Rust - If...else expression 
// 1. Conditions - if...else
// 2. Repeating actions - loops


// If Else [If Expresson ... Else Expression]
#![allow(warnings)]
fn main() {
    // let age: u16 = 18;

    // if age >= 18 {
    //     println!("You are old enough to drive a vehicle.");
    // } else {
    //     println!("Maybe wait more years before you can drive? You're now {} years old ", age)
    // }

    // Sometimes we have multiple conditions with else if
// ----------
    // Multiple Conditions with Else If:
    // let number = 6;
    // if number % 4 == 0 {
    //     println!("Number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("Number is divisible by 3")
    // } else if number % 2 == 0 {
    //     println!("Number is divisible by 2");
    // } else {
    //     println!("Number is not divisible by 4, 3, or 2")
    // }

    // Last example, using if in a let statement, how so?
    let condition = false;
    // let number = if condition {5} else {"6"}; // incompatible types
    let number = if condition {5} else {6}; // compatible types, correct. 
    println!("Number: {number}")
    // With this, number will be assigned 5 or 6 based on the value of the condition

    // Now remember it's very important to ensure that the TYPES of values in EACH ARM of the if expressions are compatible. Or else, you will encounter errors when you will compile your code.
}
//  In any programming language, controlling the flow of execution is based on two things: Conditions & Repeating actions. Together they are called control flow.

