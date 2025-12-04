fn main() {
    println!("Hello, world!");
}


/*
ABOUT CARGO

Cargo is Rust’s official build system and package manager. It is installed automatically
when you install Rust using `rustup`.

What Cargo does:
1. COMPILING CODE
   - Instead of running `rustc` manually, we use:
         cargo build
     Cargo automatically finds your source files and compiles them.

2. RUNNING PROGRAMS
   - Instead of manually compiling and then running the binary, we use:
         cargo run
     This compiles (if needed) and then executes the program.

3. CHECKING FOR ERRORS
   - To quickly check code without producing a binary:
         cargo check
     This is faster than `cargo build`.

4. MANAGING DEPENDENCIES (CRATES)
   - Cargo makes it easy to add external libraries (called *crates* in Rust).
   - Dependencies are listed in the `Cargo.toml` file.
   - Cargo automatically downloads, updates, and compiles all dependencies.

5. PROJECT STRUCTURE
   - Creating a new project:
         cargo new project_name
   - Cargo creates:
         Cargo.toml     (project configuration)
         src/main.rs    (source code)
     It also sets up version control (Git) unless disabled.

6. BUILD PROFILES
   - Debug build (default): `cargo build`
   - Release build (optimized): `cargo build --release`

7. REPRODUCIBLE BUILDS
   - Cargo ensures the same dependency versions are used every time
     using `Cargo.lock`, which locks exact versions.

In summary:
Cargo automates building, testing, running, organizing, and managing dependencies
for Rust projects. It is an essential tool in the Rust ecosystem and is used in
almost every Rust project.
*/
