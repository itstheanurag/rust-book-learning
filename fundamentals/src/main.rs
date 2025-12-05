mod functions;
mod numbers;
mod shadowing;
mod variables;

const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;

fn main() {
    println!("Hello, world!");
    println!("{}", THREE_HOURS_IN_SECONDS);

    println!("--- ABOUT THE NUMBERS ---");
    numbers::play_with_nums();
    println!("--- ABOUT THE SHADOWING ---");
    shadowing::shadowing();

    println!("--- ABOUT THE VARIABLES ---");
    variables::play_with_vars();

    println!("--- ABOUT THE COMPOUND VARIABLES ---");
    variables::compount_variables();

    println!("--- ABOUT THE SCALAR VARIABLES ---");
    variables::scalar_variables();

    println!("--- STARTING WITH FUNCTIONS ---");
    let sum = functions::add_two_nums(5, 10);
    println!("Sum is: {}", sum);

    let product = functions::multiply(4, 5);
    println!("Product is: {}", product);

    functions::print_message("Hello from functions!");

    let (min, max) = functions::min_max(10, 20);
    println!("Min: {}, Max: {}", min, max);

    functions::print_any(vec![1, 2, 3, 4, 5]);
    let fact = functions::factorial(5);
    println!("Factorial of 5 is: {}", fact);

    let squared_value = functions::square(5);
    println!("Squared value of 5 is: {}", squared_value);
    println!("Squared value of 4 is: {}",functions::SQUARED_VALUE_OF_FOUR);
}
