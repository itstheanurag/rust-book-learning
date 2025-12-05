pub fn play_with_nums() {
    println!("PLAYING WITH NUMBERS IN RUST");

    /*

    * 1. DEFAULT OVERFLOW BEHAVIOR

    * In Rust, integer overflow behaves differently depending on
    * build mode:
    *
    * - Debug mode  → overflow causes a panic at runtime.
    * - Release mode → overflow *wraps around* using two's complement.
    *
    * Example:
    * let x: u8 = 255;
    * let y = x + 1; // panic in debug, wraps to 0 in release
    */
    // Uncomment to see panic in debug mode:
    // let x: u8 = 255;
    // let y = x + 1;
    // println!("This will panic in debug mode: {}", y);

    /*
    * 2. checked_add() → Returns Option<T>
    * Safest method. If overflow would occur: returns None.
    */
    let a: u8 = 200;
    let b: u8 = 100;

    let checked = a.checked_add(b);
    println!("\nchecked_add result: {:?}", checked);
    /*
     * Output:
     * - Some(value) if safe
     * - None if overflow would occur
     * Useful when: You want to avoid crashes and handle failures gracefully.
     */
    /*

    * 3. wrapping_add() → Wraps around on overflow

    * Behavior is the same as release mode behavior:
    * 255 + 1 → 0 for u8
    */
    let wrapping = a.wrapping_add(b);
    println!("wrapping_add result (wraps on overflow): {}", wrapping);

    /*
    * 4. saturating_add() → Caps at max/min value
    * Behavior:
    * - If result > max → returns max
    * - If result < min → returns min
    */
    let saturating = a.saturating_add(b);
    println!("saturating_add result (caps at MAX): {}", saturating);

    /*

    * 5. overflowing_add() → Returns (value, overflow_flag)

    * Returns tuple:
    * (wrapped_value, true if overflow occurred)
    */
    let (overflow_val, did_overflow) = a.overflowing_add(b);
    println!(
        "overflowing_add → value: {}, overflow: {}",
        overflow_val, did_overflow
    );

    /*

    * 6. Using try_into() for safe conversions

    * try_into() returns Result<T, E>
    *
    * Example: converting i32 to u8 may fail.
    */
    let big_num: i32 = 1000;

    let safe_conversion: Result<u8, _> = big_num.try_into();

    println!("try_into conversion of 1000 → {:?}", safe_conversion);
    /*
     * Result:
     * - Ok(value)   → if within valid range
     * - Err(_)      → if overflow would occur
     *
     * Best for: integer conversions with safety
     */

    /*
     * 7. Manually preventing overflow using conditions
     */
    let x: u8 = 250;
    let y: u8 = 20;

    let manually_safe = if let Some(sum) = x.checked_add(y) {
        sum
    } else {
        println!("Manual check: Overflow avoided!");
        255
    };

    println!("Manual safe sum: {}", manually_safe);
    println!("\nFinished demonstrating all overflow-safe operations!");
}

/*
 * Summary of Methods
 *
 * Method              | Behavior
 * ---------------------------------------------------------
 * checked_add()       | Returns None on overflow
 * wrapping_add()      | Wraps around using two’s complement 
 * saturating_add()    | Caps at min/max
 * overflowing_add()   | Returns (wrapped_value, overflow_flag)
 * try_into()          | Fails safely on overflow
 * debug + normal add  | Panics on overflow
 * release + normal add| Wraps silently
 */
