// Ownership, Borrowing and Reference

// Ownership.
// ---------

// There are alot of programming languages that control the memory like C and C++ , they allow you to reserve a part of the memory and when you finished using this part of memory, it lets you release or free this part of memory.
// The problem with this system that it creates bugs because you might freed the memory more than once, or even forgot to free that chunk of memory. This is of course for the languages that let you totally control the memory. 

// This issue was solved by some programming languages by introducing the garbage collector.

// C, C++ -> Memory Management Control Issue
// The garbage collector has a role of reserving the data into memory, and once the programmer is done dealing with this data, the garbage collector releases this part of memory.
// Garbage Collector solved this issue, but created a new issue.
// If now this operation is done at the runtime in the background, and this is the main drawbacks of garbage collection. If it wants to clean up the memory it will stop the program. This is the freezing which happens for a few seconds in your program. So blame it on the garbage collector as it stops the program to clean up the memory and when it's done cleaning, the program resumes working.
// [stopping/resuming the programmg]
// This actually has created a slow performance and inefficient outcome, not recommended for those applications that need powerful memory resources.

// Let's see how rust has solved this issue by new concept called Ownership.
// National Cyber Director urges developers to move to memory safe languages such as Rust as soon as possible.
// We have known for ages that we should use memory safe languages like Rust, but after lifetimes of deploying everything in C and C++, it's not easy to do so.
// Even the White House recommends you using Rust for memory safety. Feb 2024.


// In rust every value has a single owner and there can only be one owner at a time. Actually, ownership rules help manage memory efficiently and prevent common bugs.
// ... As we are talking of ownership we will have to talk about borrowing and references.
// Borrowing in a nutshell, allows you to temporarily borrow references to values, and this actually enables safe concurrent access without sacrificing the memory safety.
 
// Ownership Rules
// ---------------
// 1. - Each value in Rust has a variable that's its owner
// 2. - There can be only one owner at a time.
// 3. - When the owner goes out of scope, the value will be dropped.
// These rules are actually in the rust book https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules
// Keep them in your mind. 

// Example: Each value in Rust has a variable that's its owner

// fn main() {
//     let s1 = String::from("Rust");
//     let len = calculate_length(&s1);
//     // Notice, we are not going to pass the s1, we are passing the reference to that string, so it actually borrowed s1 by this reference.
//     println!("Length of '{}' is {}", s1, len);
// }


// If we'll create a function that will calculate the length of the string, and this function takes s, and this s is the input (the striing), but it's not going to be the values of that string, it's going to be simply a reference to that string.
// So any reference is going to be preceded by an & ampersand sign, and it's going to result into usize or unsigned size

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// So rust has accessed the strings' data without taking the ownership, the ownership of the string remains with the s1.

// ------
// 2. - There can only be one owner at a time.
// fn main() {
//     let s1 = String::from("Rust");
//     let s2 = s1;

//     println!("{}", s2);
// }
// what we have done here is that we have actually transferred the ownership of that string, being s1 to a new variable called s2.
 
// There can only be one owner at a time.


// --------
// 3. - When the owner goes out of scope, the value will be dropped.
fn main() {
    let s1 = String::from("Rust");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}", s1, len); 
}
// s1 goes out of scope and its value will be dropped.
// fn printLost(s: &String) {
//     println!("{}", &s1);
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}
