mod types;

const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;

fn main() {
    println!("Hello, world!");
    about_variables();
    const_variables();
    shadowing();
    println!("{}", THREE_HOURS_IN_SECONDS);
    types::data_types();
}

fn about_variables() {
    mutable_variables();
    non_mutable_variale()
}

fn mutable_variables() {
    println!("---Mutable Variables---");
    let mut age = 27; // this is a variable that can be mutated
    println!("Your age is {}", age);
    age = 25;
    println!("Your updated age is {}", age);
}

fn non_mutable_variale() {
    println!("---non_mutable_variale---");
    let age = 27;
    println!("Your age is {}", age);
    // age = 25; throws error
}

fn const_variables() {
    const AGE: i32 = 32;
    println!("YOU ARE {}", AGE);
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    println!("shadowing {}", x);
    shadowing_nested();
    multi_shadow();
}

fn shadowing_nested() {
    let x = 5;
    {
        let x = 10; // shadows outer x
        println!("inner x = {}", x); // 10
    }
    println!("outer x = {}", x); // 5
}

fn multi_shadow() {
    let x = 1;

    {
        let x = x + 1; // inner x = 2
        println!("level 1: {}", x);
        {
            let x = x * 10; // deeper x = 20
            println!("level 2: {}", x);
        }
        println!("back to level 1: {}", x); // still 2
    }

    println!("outer level: {}", x); // still 1
}

/* NOTES

  VARIABLES:
  ----------
  VARIABLES CAN BE MUTATED AND YOU DON'T NEED TO SPECIFY THE TYPE OF THE VARIABLE.
  The type is inferred when you declare variables with `let`.
  Variables declared with `let` are always block scoped, meaning they only live
  inside the `{}` block in which they are created.

  IMMUTABILITY:
  -------------
  By default, variables in Rust are immutable.
  If you want to change the value of a variable, you must use the `mut` keyword.

      let mut x = 10;
      x = 20; // allowed

  Without `mut`, Rust will throw a compile-time error, forcing safer code.

  SHADOWING:
  ----------
  Rust also allows *shadowing*, where you declare a new variable with the same name.

      let x = 5;
      let x = x + 1;

  This is not the same as mutating — it creates a new variable instead of modifying the old one.

  TYPE INFERENCE:
  ---------------
  Rust automatically infers variable types, but you can explicitly specify types when needed:

      let age: u32 = 27;

  This is useful when Rust can’t determine the type by itself or for clarity.

  CONSTANTS:
  ----------
  `const` declarations MUST HAVE A TYPE, and they CANNOT BE MUTATED.
  The value of a `const` must be known at compile time.

      const MAX_POINTS: i32 = 100_000;

  Constants are not block-scoped like `let` variables.
  They have a static lifetime and are accessible everywhere in the program
  (as long as they follow visibility rules).

  NAMING:
  -------
  Constants use SCREAMING_SNAKE_CASE by convention:

      const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;

  VARIABLES VS CONSTANTS:
  -----------------------
  - Use `let` for values that may vary or are determined at runtime.
  - Use `const` for fixed values known at compile time.
  - `let` variables can be mutable (`mut`), `const` variables can never be mutable.
  - `let` is block-scoped; `const` behaves like a global, compile-time value.

*/
