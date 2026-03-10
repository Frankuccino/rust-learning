// Variables and Mutability
// In Rust variables are immutable by default
// Immutable means you cannot change it. So immutability means the inability to change a certain variable, once it's initialized or declared, you cannot change its value, by default. 
// take in mind, i'm talking about only variables, I haven't talked about constants yet. That would be the next lesson.


fn main() {
    println!("Hello, world!");
    let mut a: u16 = 5;
    println!("The value of a is {}", a);
    a = 10; 
    println!("The value of a is now {}", a);
}

// When a variable is immutable, its value cannot be change after it's assigned. And if you try to do so, you'll have a compilation error, and that's exactly what the rust book tells us https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

// So let's say you want to declare a variable and as you know in Rust, there is type annotation, you can do is as let a: u16 = 5; the 'u16'.
// In variables, it's not obligatory to do that, if you will leave it as let a = 5; it will just compile fine.
// But it tells you here that if this is intentional, prefix it with an _

// So that's it, in order to change an immutable variable in Rust, you have to add the keyword 'mut' to change it from immutable to mutable variable and then you can assign different value if you want.