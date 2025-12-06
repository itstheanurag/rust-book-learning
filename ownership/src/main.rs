fn main() {
    println!("learning about the ownership in rust");
    string_slice();
    basic_string();

    let s1 = String::from("hello"); // `s1` owns the string.
    let s2 = s1; // Ownership is moved to `s2`.
    // println!("{}", s1); // Error: `s1` is no longer valid.

    let s3 = s2.clone(); // Explicitly clone the string.
    println!("{}", s3); // `s3` owns a copy.

    borrow_example(&s3); // Borrow the string (does not take ownership).
    println!("{}", s3); // `s3` is still valid here.
}

fn borrow_example(s: &String) {
    println!("Borrowed string: {}", s);
}

fn string_slice() {
    // &str is a type of string slice.
    let s: &str = "Hello World";

    {
        let x: &str = "something inside the blocked scope";
        // x is avaiable in this scope
        println!("X is: {}", x)
    } // x scope ends here

    println!("String slice: {}", s);
}

fn basic_string() {
    // strings by default are immutable
    let mut s: String = String::from("Gaurav");
    println!("Value before pushing: {}", s);
    s.push_str(" Kumar");
    println!("Value after pushing: {}", s)
}

/*
 * OWNERSHIP: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
 *
 * Ownership is the set of rules that govern how a rust program manages memory.
 * All programs have to manage memory, some languages have automatic garbage collection that handles memory management for you.
 * Rust takes a different approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.
 * None of the ownership features slow down your program while it’s running.
 *
 */

/*
 * STACK AND HEAP
 * Usually in most of the languages, data is stored either on the stack or the heap. and most of the time you don't need to think about it.
 * But in rust, depending where you store your data, it affects the way program behaves.
 *
 * Stack:
 * - Faster access
 * - Fixed size and it must be known before hand so it's available at compile time.
 * - Data is stored in a last-in, first-out manner
 *
 * Heap:
 * - Slower access
 * - Dynamic size, can be allocated and deallocated at runtime.
 * - Data is stored in a first-in, first-out manner
 *
 */

/*
 * OWNERSHIP RULES:
 * 1. Each value in rust has a variable that’s called its owner.
 * 2. There can only be one owner at a time.
 * 3. When the owner goes out of scope, the value will be dropped.
 */
