// Error Handling Techniques [2 Approaches]


// Approach 1 
// enum Option<T> { // Defining the generic Option type
//     Some(T), // Represents a value
//     None, // Represents no value (Absence of a value)
// }
// Understanding Option T is a very important tool that you can use in order to handle errors efficiently.
// Option<T> basically is an enum. We have studied enum in the last lesson and this enum is used when a value might not be present. So it's going to avoid the pitfalls of null references which is common in other programming languages

// The way to define an option is by typing the keyword enum before that Option<T>
// And we will have two variants, We will have some(T) that takes T, and None

// Explanation:
// The Option will check something and if this something is positive, it's going to return some with that type, and if that something that we are checking is not present, none will be returned

// Approach 2 
// enum Result<T, E> { // Define the generic Result type
//     Ok(T), // Represents a value
//     Err(E), // Represents an error
// }
// Second appraoch is Result, also is an enum
// So this result type is used for operations that can succeed with the Ok(T), if it's successfull it's going to return you Ok with the type or in case of failure, it's going to return an error.
// This is particularly very useful for more critical errors where you need to specify why an operation failed and help you even to prevent it at compile time.

// Quick example on:
// Option<T>
// fn main() {
//     let result = divide(10.0, 2.0);
//     match result {
//         Some(x) => println!("Result: {}", x),
//         None => println!("Cannot Divide by Zero!"),
//     }

// }

// enum Option<T> { // Defining the generic Option type
//     Some(T), // Represents a value
//     None, // Represents no value (Absence of a value)
// }
fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// -----------------------------
// Quick Example on Approach 2:
// enum Result<T,E> {    Define the Generic Result type
//  Ok(T),               Represents a value
//  Err(E)               Represents an error
// }

fn divide_result(numerator: f64, denominator: f64) -> Result<f64,String> {
    
    if denominator == 0.0 {
        Err("Cannot Divide by 0".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    // Option<T>
    let result = divide_option(10.0, 2.0);
    match result {
        Some(x) => println!("Divide Option Result: {}", x),
        None => println!("Cannot Divide by Zero!"),
    }

    // Result<T,E>
    match divide_result(100.23, 0.0) {
        Ok(result) => println!("Divide Result: {}", result),
        Err(e) => println!("Error: {}",e),
        } 
}