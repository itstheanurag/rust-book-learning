/*
 * add_two_nums
 *
 * This is a simple function that:
 * - Accepts two unsigned integers (u32)
 * - Prints the values being added
 * - Returns their sum
 *
 * Important Notes:
 * - The use of `return` is optional; Rust implicitly returns the last expression.
 * - u32 cannot be negative and has a max value of 4,294,967,295.
 * - Overflow behavior: debug → panic, release → wrap.
 */
pub fn add_two_nums(a: u32, b: u32) -> u32 {
    println!("Adding {} and {}", a, b);
    return a + b;
}


/*
 * multiply
 *
 * This function:
 * - Takes two signed integers (i32)
 * - Returns the product
 *
 * Key Notes:
 * - Unlike u32, i32 supports negative values.
 * - Demonstrates a function with no explicit `return` keyword.
 * - Last expression without semicolon becomes the returned value.
 */
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}


/*
 * print_message
 *
 * A simple function that:
 * - Takes a borrowed string slice &str
 * - Prints a message to the console
 *
 * Notes:
 * - No return type is specified, so the return type is unit `()`.
 * - `&str` is an immutable string reference; cheap and efficient to pass.
 */
pub fn print_message(msg: &str) {
    println!("Message: {}", msg);
}


/*
 * min_max
 *
 * This function:
 * - Takes two integers
 * - Returns a tuple of (min_value, max_value)
 *
 * Notes:
 * - Demonstrates returning multiple values using a tuple.
 * - Uses built-in `.min()` and `.max()` methods from Rust's standard library.
 * - Tuples are useful for lightweight data grouping without creating structs.
 */
pub fn min_max(a: i32, b: i32) -> (i32, i32) {
    (a.min(b), a.max(b))
}


/*
 * print_any
 *
 * A generic function that:
 * - Accepts ANY type T that implements the Debug trait
 * - Prints the value using {:?}
 *
 * Notes:
 * - Demonstrates Rust generics and trait bounds.
 * - Debug is required because println!("{:?}") needs it.
 * - Works for integers, strings, vectors, custom structs (if Debug is derived).
 *
 * Example:
 * print_any(42);
 * print_any("hello");
 * print_any(vec![1,2,3]);
 */
pub fn print_any<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}


/*
 * factorial
 *
 * Recursive function that:
 * - Computes factorial of n (n!)
 * - Uses classic recursion: n * factorial(n-1)
 * - Base case: factorial(0) = 1
 *
 * Notes:
 * - Only works for small n because recursion depth is limited.
 * - Multiplying numbers can overflow u32 easily:
 *   12! = 479M (fits)
 *   13! = 6.2B (overflows u32!)
 * Rust does NOT automatically detect overflow in release mode.
 */
pub fn factorial(n: u32) -> u32 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}


/*
 * square (const fn)
 *
 * A `const fn` is a function that:
 * - Can be evaluated at compile time when used in const contexts
 * - Must follow strict rules: no heap allocation, no loops (until stable), no mutation
 *
 * Notes:
 * - The calculation is done entirely at compile time if used in a const value.
 * - Useful for constants that depend on computations instead of hardcoded values.
 */
pub const fn square(x: i32) -> i32 {
    x * x
}


/*
 * SQUARED_VALUE_OF_FOUR
 *
 * A constant computed at compile time using a const function.
 *
 * Notes:
 * - Compile-time computations improve performance and reduce runtime work.
 * - Even though square(4) is used here, Rust's dead_code lint may still warn
 *   "function square is never used" because const evaluation does NOT count
 *   as a runtime use.
 */
pub const SQUARED_VALUE_OF_FOUR: i32 = square(4);
