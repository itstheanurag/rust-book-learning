pub fn compount_variables() {
    /*
     * Compound types in Rust include tuples and arrays.
     *
     * They allow grouping multiple values together:
     * - Tuples: can contain values of DIFFERENT types
     * - Arrays: contain values of the SAME type, fixed length
     */

    println!("--- TUPLE EXAMPLE ---");
    tuple_example();
    println!("--- ARRAY EXAMPLE ---");
    array_example();
}

fn tuple_example() {
    /*
     * UNIT TYPE: ()
     * The unit `()` is a special tuple with ZERO elements.
     *
     * - It is the return type of functions that don't return anything.
     * - It has exactly ONE possible value: `()`.
     * - It is used when "a value is needed but no meaningful value exists".
     *
     * Memory-wise:
     * - It takes zero bytes.
     * - Rust uses it for functions like `fn main()`.
     */
    let tup: () = (); // unit type value

    println!("Unit type value: {:?}", tup);

    /*
     * TUPLE WITH MULTIPLE VALUES
     * Tuples allow grouping different types into a single value.
     *
     * Here, we create a tuple representing a person:
     * - &str     (name)
     * - char     (gender)
     * - &str     (profession)
     * - i32      (age)
     * - f64      (height)
     *
     * Tuples are stored on the stack with all elements placed contiguously.
     */
    let human = ("Gaurav", 'm', "Developer", 27, 5.8);

    /*
     * DESTRUCTURING A TUPLE
     * Rust allows breaking a tuple into individual variables.
     *
     * The pattern:
     * let (a, b, c) = tuple;
     *
     * This assigns:
     * - name       ← "Gaurav"
     * - gender     ← 'm'
     * - profession ← "Developer"
     * - age        ← 27
     * - height     ← 5.8
     *
     * Each extracted value gets its own variable.
     */
    let (name, gender, profession, age, height) = human;

    println!("Name: {}", name);
    println!("Gender: {}", gender);
    println!("Profession: {}", profession);
    println!("Age: {}", age);
    println!("Height: {}", height);
}

fn array_example() {
    /*
     * ARRAYS IN RUST
     * Arrays store multiple values:
     * - All elements MUST be the same type.
     * - Arrays have a FIXED length known at compile time.
     *
     * Type syntax:
     *      [T; N]
     * T = element type
     * N = number of elements
     *
     * Example:
     * let numbers: [i32; 5] = [1,2,3,4,5];
     *
     * Memory:
     * - Arrays are stored on the stack as a contiguous block.
     * - Fast and predictable memory layout.
     */
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    /*
     * ITERATING THROUGH AN ARRAY
     * `.iter()` returns an iterator over references (&i32).
     *
     * Why references?
     * - It prevents moving values out of the array.
     * - Arrays need their elements intact after the loop.
     *
     * Each iteration prints the current element.
     */
    for number in numbers.iter() {
        println!("Number: {}", number);
    }

    /*
     * Additional notes:
     *
     * You can also access elements by index:
     * numbers[0] → 1
     *
     * But Rust performs bounds checking at runtime:
     * accessing numbers[100] will cause a panic.
     *
     * Safer alternative:
     * numbers.get(100) → returns Option<&T>
     */
}
