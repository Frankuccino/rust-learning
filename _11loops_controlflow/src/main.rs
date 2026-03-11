// Repetition with Loops
// Doing Things Over and Over
// 3 Types of Loops from Rust:
// Loop
// While Loop
// For Loop
// fn main() {
//     // Loop keyword
//     // loop {
//     //     println!("Hello, World!");
//     // }

//     // While loop
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("The result is {result}");

//     // Loop Labels to disambiguate between multiple loops
//     // When you deal with loops nested within one another, that's when you use the loop labels.
//     // The break and continue will apply to the innermost loop by default. 

// }


// fn main() {
//     let mut count = 0;
//     'counting_up: loop { // Outer Loop
//         println!("Count = {count}");
//         let mut remaining = 10;

//         loop {  // Inner Loop
//             println!("Remaining = {remaining}");
//             if remaining == 9 {
//                 break; // Break inner loop
//             }
//             if count == 2 {
//                 break 'counting_up; // break outer loop
//             }
//             remaining -= 1;
//         };
//         count += 1; // increment count in outer loop.
//     };
// }

// While loop
// The while loop runs the loop as long as the condition holds true.

fn main() {
    let mut number = 3;
    while number != 0 {
        println!("Number is {number}");
        number -= 1;
    }

    // Looping through a Collection with for...loop
    let a = [1,2,3,4,5,6];
    let b = ["a", "b","c","d","e","f"];
    for element in a {
        println!("{element}");
    };
    for letter in b {
        println!("{letter}");
    };
}