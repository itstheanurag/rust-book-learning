mod numbers;
mod shadowing;
mod types;
mod variables;

const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;

fn main() {
    println!("Hello, world!");
    variables::play_with_vars();
    println!("{}", THREE_HOURS_IN_SECONDS);
    types::basics();
    numbers::play_with_nums();
    shadowing::shadowing();
}
