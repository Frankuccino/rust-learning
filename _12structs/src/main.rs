// Structs
// structs are used to name and package together related values similar to tuples

fn main() {
    // tuple
    // let rectangle = (200,500);
    // this is the rectangle tuple, it has a width and height.

    // Unlike tuples, each field in a struct is named. This provides clear identification of what each value represents. So this feature makes struct unique more flexible and expressive compared to something like tuples.
    // Data is basically accessed solely if you will order it.

    // Struct
    struct Book {
        title: String,
        author: String,
        available: bool, 
        pages: u32,
        rating: u32,
    }

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64    
    }

    let mut user = User {
        active: true,
        username: String::from("Frankuccino"),
        email: String::from("frank@gmail.com"),
        sign_in_count: 1,
    };

    // frank_user.sign_in_count =+ 1;
    // user.username = String::from("Furianko");
    println!("username: {}, active: {}, email: {}, sign_in_count: {}", user.username, user.active, user.email, user.sign_in_count);


    // Return a Struct from a function
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            email,
            username,
            sign_in_count: 1
        }
    }
    // The example here uses a 'field init' shorthand which simplifies the code when variable names match the struct filed names.
    // From a function, you can actually return a User, and do not forget the return '-> User'

    // Create Instances from other instances
    let user2 = User {
        email: String::from("another@m.com"),
        ..user
    };
    println!("User2 email is {}, username is {}", user2.email, user2.username);

    // Tuple Structs - They are like normal structs but the main difference is that they do not have any name fields. So you can create a double struct by saying ...
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32); 
    
    let black = Color(0,0,0);
    let white = Color(255,255,255);

    // Unit-Like Structs - Unit-like structs are very specific, they have no fields and are used when you need a type to implement a trait but don't need to store data.
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

// Note that the entire struct instance must be mutable in order to modift any of the fields inside. 

// So the lists of structs mentioned in this lesson are:
/*
- Struct: the Standard "named field" version
- Instances: Bringing the blueprint to life with data.
- Struct Update Syntax: Efficiently copying field from one instance to another (..)
- Tuple Structs:  Structs that look like tuples but have a name (e.g. Color(i32,i32,i32) ).
- Unit-like Struct: Struct with no field at all, often used for defining traits on a type.


*/


// What's not mentioned in this lesson was about impl which is adding logic to those mentioned structs. impl (implementation) blocks is when you define two types of functions: Methods and Associated Functions.

// Methods -> Methods are functions that operate on a specific instance of a struct
//  - the 'self' keyword: the first parameter must be self or (&self / &mut self), representing the instance the method is called on.
// Accessing Data: they use the dot operator to access fields ofthat instance.
// syntax example: rect.area()

// 2. Associated Functions
// Associated functions are linked to the struct type itself, not a specific instance. 
//  
// No self: They do not take self as their first parameter.
// Common Use: They are most often used as constructors (like String::from or Rectangle::new).
// Syntax Example: Rectangle::new(10, 20) using the double colon ::. 

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     // Associated Function (Constructor)
//     fn new(w: u32, h: u32) -> Self {
//         Self { width: w, height: h }
//     }

//     // Method (Uses an instance)
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
