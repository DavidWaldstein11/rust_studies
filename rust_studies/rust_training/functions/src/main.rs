// Demonstrating various types of functions in Rust

use std::time::Duration;
use tokio::time::sleep; // Only needed if you're using async with tokio

#[tokio::main] // This is required for async main using the tokio runtime
async fn main() {
    println!("== Basic Functions ==");
    say_hello();
    greet_user("Alice");

    let sum = add(10, 20);
    println!("10 + 20 = {}", sum);

    print_separator();

    let square = square_of(6);
    println!("Square of 6 is {}", square);

    println!("\n== Closures ==");
    closures_demo();

    println!("\n== Lifetimes ==");
    let result;
    {
        let str1 = String::from("Rust");
        let str2 = "Language";
        result = longest(&str1, str2); // `str2` lives longer than this scope
    }
    println!("Longest string: {}", result);

    println!("\n== Async Function ==");
    async_demo().await;
}

//
// --- Basic Functions ---
//

fn say_hello() {
    println!("Hello from a function!");
}

fn greet_user(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn print_separator() {
    println!("----------------------");
}

fn square_of(x: i32) -> i32 {
    x * x // Implicit return
}

//
// --- Closures ---
//

fn closures_demo() {
    // A simple closure that adds two numbers
    let add = |x: i32, y: i32| x + y;
    println!("Closure add(2, 3): {}", add(2, 3));

    // A closure that captures an outer variable
    let factor = 10;
    let multiply = |x: i32| x * factor;
    println!("Closure multiply(4): {}", multiply(4));
}

//
// --- Lifetimes ---
//

// Returns the longest of two string slices using explicit lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//
// --- Async Function ---
//

async fn async_demo() {
    println!("Waiting for 2 seconds...");
    sleep(Duration::from_secs(2)).await;
    println!("Done waiting!");
}
