pub fn basics() {
    // Scalar types: integers, floating-point numbers, booleans, and characters

    /*
      i8   = -128 to 127
      i16  = -32,768 to 32,767
      i32  = -2,147,483,648 to 2,147,483,647
      i64  = -9.22e18 to 9.22e18
      i128 = -1.70e38 to 1.70e38
      isize = arch-dependent

      u8   = 0 to 255
      u16  = 0 to 65,535
      u32  = 0 to 4.29e9
      u64  = 0 to 1.84e19
      u128 = 0 to 3.40e38
      usize = arch-dependent


      anything outside these ranges will cause a compile-time error called integer overflow
    */

    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'R';

    // Compound types: tuples and arrays
    let tuple: (i32, f64, char) = (integer, float, character);
    let array: [i32; 3] = [1, 2, 3];

    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    println!("Tuple: {:?}", tuple);
    println!("Array: {:?}", array);
}

/*
    DATA TYPES IN RUST

Rust has two main categories of data types:

1. SCALAR TYPES
A scalar type represents a single value.
Rust has four primary scalar types:

1) INTEGERS (i8, i16, i32, i64, i128, isize, u8, u16...)
   - Whole numbers, positive or negative.
   - Example: 42 (i32 is default).

2) FLOATING-POINT NUMBERS (f32, f64)
   - Numbers with decimals.
   - f64 is the default and more precise.
   - Example: 3.14

3) BOOLEANS (bool)
   - true or false.
   - Often used in conditions.
   - Example: true

4) CHARACTERS (char)
   - Represents a single Unicode scalar value.
   - Stored as 4 bytes.
   - Example: 'R'

These types are simple, atomic values.

2. COMPOUND TYPES

Compound types can group multiple values into one.

Rust has two compound types:

1) TUPLES
   - Can hold values of different types.
   - Fixed length.
   - Example: (42, 3.14, 'R')
   - Access via destructuring or indexing (tuple.0)

2) ARRAYS
   - A list of values with the SAME type.
   - Fixed size known at compile time.
   - Example: [1, 2, 3]
   - Useful when size should not change.

SUMMARY

Scalar types = single values.
Compound types = multiple values combined in one structure.

Rust’s strict type system ensures memory safety while still
allowing efficient, predictable performance.

*/
