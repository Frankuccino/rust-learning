// Pimitive data types (also known as scalar datatypes)
// int, float, bool, char,

// Integer
// Rust has signed (+ and -) and unsigned integer (only+) types of different sizes
// i8, i16, i32, i64, i128: Signed integers
// They are signed integers, they hold positive values or negative values.
// The unsigned integers hold only positive values.
// u8, u16, u32, u63, u128: Unisgned integers.

fn main(){
  let x: i32 = -42;
  let y: u64 = 100;
  println!("Signed Integer: {}", x);
  println!("Unsigned Integer: {}", y);
// difference between i32 (32 bits) and i64 (64 bits)
// range: 
//      i32 - 2^31 
//      i64 - 2^63
// i32 - 2147483647
// i64 - 9223372036854775807 - both of these are the largest values, more than that and you'll have a compiling error.
// u32 is 2^32 and u64 is 2^64 
  let e: i32 = 2147483647;
  let i: i64 = 9223372036854775807;
  println!("Maximum value of i32: {}", e);
  println!("Maxiumum value of i64 {}", i);

  // Floats [Floating Point Types] - floats are numbers with fractional parts.
  // f32, f64

  let pi: f64 = 3.14;
  println!{"Value of pi: {}",pi}

  // Boolean Values:
  let is_snowing: bool = true;
  println!("Is it snowing? {}", is_snowing);

  // Character Type - char - This actually represents a single uni-code scalar value
  let letter: char = 'a';
  println!("First letter of the alphabet is {}", letter);
}

