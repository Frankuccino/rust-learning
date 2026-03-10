// Shadowing
// Shadowing is not the same as marking a variable as mutable.

// Shadowing, even the word is not clean, what shadow is what exactly? and what the shadow has to do in programming.
// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing

fn main() {
    let x = 5; // Result is 5

    let x = x + 1; // Result is 6

    {
        let x = x * 2; // Result is 12
        println!("The value of x in the inner scope is: {x}");
    }
    
    println!("The value of x in main function is: {x}");
    // A unique effect of shadowing in Rust.
    let spaces = "    "; // 4 spaces, empty btw. this is a string
    let spaces = spaces.len(); // this is now a usize
    println!("Spaces in spaces is: {}", spaces);
}
// Basically what you have done, the second variable, the x after after the assignment operator...
// the x = 'x' + 1 which actually is the previous x so both of them are old.  
// the 'x' = x + 1 is the new x so the first was shadowed by the 2nd x. The first variable with the same name was shadowed by the second variable with the same name, and when you print x, the compiler will only read the 2nd x or the new one. 

// Shadowing simply means that you create a variable with the same name, and you are shadowing the first one. The second variable is what the compiler will see when you use the name of the variable. The second variable here overshadows the first and takes any uses of the variable name to itself.
// You are shadowing the first by creating a second.

// One more very important point that shadowing is different from marking a variable as mut.

// A good thing about shadowing is that when the first variable instance is a number, the second one could be a different type altogether without any problem. See the example on spaces above. 

// This part is theoretical, still, it's very very important and sometimes it's boring but it's very important. You need to know those basics because without the basics you will not be able to code yourself. Okay, that's a little tip for you.