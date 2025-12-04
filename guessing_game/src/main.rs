use std::{cmp::Ordering, io};

use rand::Rng;


fn main() {
    println!("Guessing Game Started!");
    let secret_number: u32 = rand::rng().random_range(1..100);

    loop {
       println!("Please input your guess:");

       let mut guess: String = String::new();

       io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");

      // this is called shadowing
      let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_err) => {
            println!("Please enter a valid input");
            continue;
        }
      };

      println!("You guessed {}", guess );
      // match is exhaustive, need to cover all possible scenerio's otherwise it will complain
      match guess.cmp(&secret_number) {
        Ordering::Equal => {
             println!("You Guessed correct");
             break;
        } // this is an arm in the match pattern
        Ordering::Greater => println!("You guessed a number greater than machine"),
        Ordering::Less => println!("You guessed a number less than machine")
      }
    }
    
}



/* ABOUT THE IMPORTS

To get input from the user, we use the `io` module from Rust’s standard library.
Since `io` is *not* part of the standard prelude, we must import it manually using:

    use std::io;

The Rust *prelude* is a small collection of commonly used types and traits that are
automatically imported into every Rust program. Examples include `String`, `Vec`,
`Option`, `Result`, and traits like `Copy`, `Drop`, and `From`.

Rust does *not* automatically import everything from the standard library.
Modules like `io`, `fs`, `env` etc. must still be imported explicitly when needed.

*/


/* ABOUT STRING

`String` is part of Rust’s standard library and is included in the *prelude*, so we do
not need to import it manually.

A `String` is a growable, heap-allocated data structure used to store UTF-8 encoded text.
Because it is stored on the heap, its size can change at runtime.

Variables in Rust are immutable by default, so we must use `mut` to make a variable mutable:

    let mut guess: String = String::new();

`io::stdin()` returns a handle to the standard input of the program.

`read_line(&mut guess)` appends user input into the `guess` string. The `&mut guess`
is needed because the method modifies the string.

`expect("Failed to read line")` is used for error handling. If an error occurs while
reading input, the program will panic and display the provided message.

*/


/* ABOUT RANDOM NUMBERS & CRATES

The Rust standard library does NOT include a random number generator.
To generate random values, we use an external crate such as `rand`.

In Rust, external libraries are called **crates**.
A *crate* is a compilation unit—either:
  - a **library crate** (provides code for other programs), or
  - a **binary crate** (produces an executable program).

Crates cannot run by themselves unless they are binary crates. Library crates
must be used by another crate (like your main program).

Cargo (Rust’s package manager) allows adding external crates through `Cargo.toml`.

*/


/* ABOUT THE PROGRAM FLOW & WHY WE DO EACH STEP

1. STARTING THE GAME
We begin by printing "Guessing Game Started!" so the user knows the program has begun.
This is simply user feedback.

2. GENERATING A SECRET NUMBER
    let secret_number: u32 = rand::rng().random_range(1..100);

We generate a random number the user must guess.
- We store it in `secret_number` so the value persists across multiple guesses.
- It is typed as `u32` because the `rand` crate’s random functions return unsigned integers.

3. USING A LOOP
We use an infinite `loop` because:
- The user should be able to guess multiple times.
- We only break out of the loop when the user guesses correctly.

4. TAKING USER INPUT
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)...

We repeatedly ask the user for input inside the loop because:
- Each iteration represents a new guess.
- Input is stored in a `String` since user input comes as text.

5. SHADOWING THE VARIABLE `guess`
    let guess: u32 = match guess.trim().parse() { ... };

We perform *shadowing* to convert the user’s text input (`String`) into a number (`u32`).
This allows us to reuse the name `guess` but change its type.
Shadowing is preferred here because:
- It avoids creating extra variable names like `guess_str`, `guess_num`.
- It keeps the code clean and readable.

We use `match` on `parse()` because parsing can fail.  
If parsing fails, we inform the user and continue the loop without crashing.

6. COMPARING THE GUESS WITH THE SECRET NUMBER
    match guess.cmp(&secret_number) { ... }

Rust provides `cmp()` to compare values.
It returns one of the `Ordering` variants:
- `Less`
- `Greater`
- `Equal`

We use a match expression because:
- Rust requires us to handle *all possible outcomes* (exhaustive matching).
- Each arm gives appropriate feedback to the player.

7. WIN CONDITION
If the guess is equal:
- We print a success message.
- We `break` out of the loop, ending the game.

8. CONTINUE GUESSING
If the guess is higher or lower:
- We give a hint to the player.
- The loop continues, allowing another attempt.

Overall Purpose:
The program demonstrates several fundamental Rust concepts:
- Using external crates (`rand`)
- Handling user input
- Error handling with `match` and `expect()`
- Variable shadowing
- Infinite loops with conditions to break
- Using `cmp()` and pattern matching
- Working with numbers and strings
- Structuring readable logic flow

The loop continues until the user gives the correct guess, making it an interactive game.
*/
