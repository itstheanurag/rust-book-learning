/*
 * This function demonstrates Rust variable shadowing.
 *
 * Shadowing allows us to redeclare a variable with the same name using `let`.
 * - It does NOT mutate the previous variable.
 * - It creates a completely NEW binding that overrides the visible name.
 * - This enables transformation of values without requiring `mut`.
 * - Each `let x = ...` hides (shadows) the previous `x` only in its scope.
 */
pub fn shadowing() {
    /*
     * First binding of `x`.
     * `x` lives in the function scope and holds the value 5.
     */
    let x = 5;

    /*
     * Second binding of `x`.
     * This shadows the previous binding, meaning:
     * - The old `x = 5` is no longer accessible from here onward.
     * - A NEW variable named `x` is created with the value (5 + 1) = 6.
     * - This is NOT mutation; it's variable rebinding.
     *
     * Shadowing is commonly used for:
     * - transforming values
     * - changing types without mutability
     * - keeping variable names meaningful while applying steps
     */
    let x = x + 1;

    println!("shadowing {}", x); // prints 6

    shadowing_nested();
    multi_shadow();
}

/**
 * Demonstrates scope-based shadowing.
 *
 * Shadowing inside a nested block does NOT affect the variable outside it.
 * Each block can create a new variable with the same name.
 */
fn shadowing_nested() {
    let x = 5; // outer binding

    {
        /*
         * Inner scope binding.
         * This shadows the outer `x` ONLY inside this `{}` block.
         *
         * The outer `x` still exists but is temporarily hidden.
         * When this scope ends, the inner `x` is dropped.
         */
        let x = 10;

        println!("inner x = {}", x); // prints 10
    }

    /*
     * After leaving the inner block:
     * - The inner `x = 10` is gone.
     * - The outer `x = 5` becomes visible again.
     */
    println!("outer x = {}", x); // prints 5
}

/**
 * Shows multi-level shadowing in nested scopes.
 *
 * Each new `let x = ...` introduces a NEW variable that hides the previous one
 * *only within the current scope and its descendants*.
 */
fn multi_shadow() {
    let x = 1; // outermost x

    {
        /*
        First shadow in level 1:
        The new `x` takes the previous value (1) and adds 1 → becomes 2.
        The outer x (=1) is hidden but not replaced.
        */
        let x = x + 1; // x = 2 at level 1
        println!("level 1: {}", x); // prints 2

        {
            /*
             * Second shadow in level 2:
             * This shadow takes level 1's `x = 2` and multiplies it by 10 → becomes 20.
             *
             * Only this inner block sees this `x = 20`.
             * After leaving this scope, we return to the level 1 binding (`x = 2`).
             */
            let x = x * 10; // x = 20
            println!("level 2: {}", x); // prints 20
        }

        /*
         * The deepest shadow (x = 20) is gone.
         * We are back to the level 1 `x = 2`.
         */
        println!("back to level 1: {}", x); // prints 2
    }

    /*
     * The level 1 shadow (`x = 2`) is now gone.
     * We are back to the original outermost `x = 1`.
     */
    println!("outer level: {}", x); // prints 1
}
