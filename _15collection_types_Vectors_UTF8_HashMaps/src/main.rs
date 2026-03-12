use std::collections::HashMap;

// Collection Types in Rust:
// Vectors
// UTF8
// Hash Maps
// ------------ 
fn main() {
    println!("Hello, world!"); 
    // If you want to create a vector, simply you will type the keyword vec
    let v: Vec<i32> = Vec::new();
    // New here is a method, This simply tells Rust; I want to create a new vector.
    let mut the_vec: Vec<i32 > = vec![1,2,3];

    let mut the_numbers_vec: Vec<i32> = Vec::new();
    the_numbers_vec.push(5);
    the_numbers_vec.push(20);
    the_numbers_vec.push(6);
    the_numbers_vec.push(7);

    the_vec.push(4);

    println!("{:?}",v);
    println!("{:?}", the_vec);
    println!("{:?}", the_numbers_vec);

    // Reading elements - Accessing elements without ownership through reference.
    let vec_read = vec![1,2,3,4,5];
    let third_referenced: &i32 = &vec_read[2]; // Direct Indexing
    println!("The third element that's accessed through reference is {third_referenced}");

    // Another way of returning elements of a vector - Using a get method
    let fourth_get = vec_read.get(3); // Index 3 which is the 4th el.
    // This method basically gets whatever you want from the vector
    match fourth_get {
        Some(fourth) => println!{"The fourth element from a get method is {fourth}"},
        None => println!("There is no fourth element!")
    }
    utf_8();
    hash_maps();
}
//--------------
// You can think about vector as a cool dynamic array that can grow or shrink as needed.
// When you call this 'vec::new()', what you have done is that you have said; Hey Rust, I want to make new empty vector here. So more often you will start with some values in your vector.
// Actually rust provides a handy macro for this called 'vec!'

// Let's say you want to add elements to your vector, you can do this using the push method. It's like in JS or python, push, append methods that they can push something into an array.

// The most important, the vector should have homogeneous type of elements.

// Vectors really are awesome, and what's more awesome is how you can read elements of vectors.
// This is going to go through indexing, not only that but we can have another method called 'get'.
// we can actually return any element inside by indexing. So we have 0 index-based system in programming in general.

// -------------------------UTF 8-------------------------------------
// UTF-8 is all about strings - Strings in Rust are very important
// You know that string is a chain of characters
// Rust only has one string type in the core language, which is the string slice '&str'. String slice is usually seen in its borrowed form which are references to some UTF-8 encoded string data stored elsewhere. String literals, for example, are stored in the program's binary and are therefore string slices.

// The string type, which is provided by Rust's standard library rather than coded into the language, is growable, mutable, owned, UTF-8 encoded string type.

// So you have the string slice, it's a reference to a string that you have written it or encoded it by hand.
// If you do String::from("Hello") - This is the String
// To create a new string; let mut s = String::new();
// You have to remember that Strings are UTF-8 encoded 

fn utf_8() {
    // 1st way
    let s1 = "whatever".to_string();
    // 2nd
    let s2 = String::from("whatever heh");
    // 3rd - mutable variable
    let mut s3 = String::from("foo");

    s3.push_str("bar"); // The difference here is that we are pushing a string slice and not character.
    s3.push('s' ); // Take note, this is a char, and we only wrap the char with a single quotation mark '' 
    println!("The value of s1 is: {}", s1);
    println!("The value of s2 is: {}", s2);
    println!("The value of s3 is: {}", s3);

    // If you want to combine strings, you use the + operator. (concatination with the + operator on strings)
    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5; // Note that s1 has been moved here and can no longer be used - This means we transfered the ownership of s4 to this variable s6 so that we can combine it with a referenced value s5.
    println!("The value of s6 is: {}", s6);
    // The + operator actually uses a method which signature looks like this fn add(self, s &str) -> String {} ... The 1st parameter 'self'  there is owned variable, and 2nd parameter is a string slice '&str' a reference to an owned variable.
    // In other words, we should add/concatinate an owned, and borrowed variables.

    // Using 'format!' to concatinate
    let greeting = String::from("Hello, my name is");
    let name = String::from("Frank");
    let full_message = format!("{greeting} {name}");
    println!("{full_message}");
}

// Read and understand the docs of Rust, trust me it is good and complete.
// https://doc.rust-lang.org/book/ch08-02-strings.html

// ------------------------ Hash Maps-----------------------------
fn hash_maps() {
    let mut scores = HashMap::new();
    scores.insert(string("Blue"), 10);
    scores.insert(string("Yellow"), 50);

    let team_name = string("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score of blue team is: {score}");

    // Iterating over the hash maps
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // Rust by default, use a very secure hashing algorithm to protect against the DOS attacks.

}

fn string(string: &str) -> String {
    String::from(string)
}

