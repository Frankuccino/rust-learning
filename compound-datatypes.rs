// Compound Data Types
// arrays, tuples, slices, and strings (slice string)

// Arrays - Arrays are a fixed size collection of elements of the same type of homogeneous types.  An array of the same datatype on each of the elements in the array.

fn main(){
  let numbers: [i32; 5]  = [1,2,3,4,5];
  println!("Number Array: {:?}", numbers);
  // let mix = [1,2,"apple", true];
  // println!("Mix Array: {:?}", mix);
  let fruits: [&str;  3] = ["Apple", "Banana", "Orange"];
  println!("Fruits Array: {:?}", fruits);
  println!("Fruits Array 1st element: {}", fruits[0]);
  println!("Fruits Array 2nd element: {}", fruits[1]);
  println!("Fruits Array 3rd element: {}", fruits[2]);
  // /////////////////////////////////////////////////

  // Tuples - Tuples contain heterogeneous collection of elements of fixed size.
  let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
  println!("Human Tuple: {:?}", human);

  let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
  println!("My Mix Tuple: {:?}", my_mix_tuple);

  // Slices - Slices simply are dynamically sized view into a contagious sequence of elements. In computer science specifically in programming, contagious sequence of elements is a very well known term especially when it comes to memory.

  // Slices: [1,2,3,4,5] - so contangious means uninterrupted. Adjacent to one another. So when disaplying something in a contagious sequence form like that array. The memory doesn't have to jump between memories, but rather going 1 next 2, to two to 3rd, then 4th, 5th,... This is very good for memory allocation and memory efficiency.

  let number_slices:&[i32] = &[1,2,3,4,5];
  println!("Number Slice: {:?}", number_slices);
  // This has the same idea of string slice.
  // WE have a number slice, and this number slice has an array, infront of it we have that ampersand (&)
  let animal_slices:&[&str] = &["Lion","Elephant", "Crocodile"];
  println!("Animal Slice: {:?}", animal_slices);
 
  let book_slices:&[&String] = &[&"IT".to_string(),&"Harry Potter".to_string(), &"ZEN".to_string()];
  println!("Book Slice: {:?}", book_slices);
  
  // Strings vs String Slices (&str)
  // Strings [growable, mutable, owned string type]
  // The main difference between strings and string slices is that, the strings are growable, which means expandable. You can increase them or decrease them, and hence, they are mutable, you can push and delete data from a certain variable if you want, and they are owned string types. Owned means they are not borrowed.

  // You need to know that memory allocation and memory efficiency is very important in rust programming language. Rust is basically C and C++ but much much better, it has the best of all worlds from C and C++ as they are low level languages, they are very very fast because they're very close to your hardware. Rust actually does the same thing but with automatic memory management and no garbage collection. 
  // When you declare a variable which is a string, actually it's going to be allocated on the heap, and there's a difference between the heap and the stack... They are actually allocated on the heap and then these string objects can grow or shrink in size as needed dynamically, it's not fixed, but dynamic, it can grow, it can shrink, you can add to it, you can delete from it. This is done at the runtime in the background. 
  // Memory allocation is made dynamically in the heap, but that's actually very very slow to access because, you know it's not fixed and it's wide, it's big, and it's slow if you have a lot of strings  and stored in the heap, but they are very useful because sometimes you want to modify your own string data dynamically and that's your only choice basically.

  // You need to know that any/all data type in Rust by default is immutable, which means that you cannot change it. 
  let mut stone_cold: String = String::from("Hell, ");
  stone_cold.push_str("Yeah!");
  println!("Stone Cold Says: {}", stone_cold);
  // So string is mutable (adding the mut key word before the variable name), it's growable, it's an owned string stored on the heap unlike the string slice which is stored on the stack.

  // B- &str (String Slice) - String slice which is a reference. this not an owned string, this is a reference to a portion of string that it's stored somewhere in your code. They're also immutable, which means that you cannot modify anything.
  // The String slices are used to reference string literals or substrings of string objects without needing to copy or own the data.
  // This is very good for memory efficiency because you do not have to copy the same variable. These string slices are used when you want to work with string data without taking ownership of it.
  // They also have specific size, and no number of bytes to the stack. So the stack remembers very well and reacts very quickly in contrast to the heap which is expandable and dynamic, it goes at the runtime at the backgroun and can grow bigger and bigger and can be slower and slower. 
  // The stack is quicker, the heap is slower, but the stack can't have a mutable data types, while the heap can have a dynamic mutable data types.

  let hello_world: String = String::from("Hello, World");
  let slice: &str = &hello_world[0..5];
  // we haven't copied the value of hello world, as it is only referencing to the variabe hello_world. It's not copied, only referred to the value that's inside that hello_world variable.
  println!("Slice Value: {}", slice);

  // One last thing:
  // Rust cleans any memory allocated to any variable.
  // If you try to display that slice outside here, you will get an error.
}
// fn print(){
//   println!("Slice: {}", slice);
// }



// Types of formats in Rust:
// 1. Debuggable Format
// 2. Display Format 