use std::time::Duration;
use tokio::time::sleep;

// ==============================
// 🔷 MAIN
// ==============================
#[tokio::main]
async fn main() {
    println!("\n== Ownership: Stack vs Heap ==");
    ownership_demo();

    println!("\n== Ownership: Move, Clone, Borrow ==");
    move_clone_borrow_demo();

    println!("\n== Basic Functions ==");
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
        result = longest(&str1, str2);
    }
    println!("Longest string: {}", result);

    println!("\n== Async Function ==");
    async_demo().await;
}

// ==============================
// 🔷 OWNERSHIP EXAMPLES
// ==============================
fn ownership_demo() {
    // Stack data
    let x = 42;
    let y = x; // Copy
    println!("Stack: x = {}, y = {}", x, y);

    // Heap data
    let s1 = String::from("hello");
    let s2 = s1; // Ownership moved
    println!("Heap: s2 = {}", s2);

    // Clone to keep both
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("Cloned: s3 = {}, s4 = {}", s3, s4);
}

fn move_clone_borrow_demo() {
    let name = String::from("Rustacean");

    // Immutable borrow
    print_name(&name);
    println!("Original after borrow: {}", name);

    // Mutable borrow
    let mut text = String::from("Hi");
    add_rust(&mut text);
    println!("After mutation: {}", text);
}

fn print_name(n: &String) {
    println!("Borrowed name: {}", n);
}

fn add_rust(s: &mut String) {
    s.push_str ", Rust!");
}

// ==============================
// 🔷 FUNCTIONS
// ==============================
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
    x * x // implicit return
}

// ==============================
// 🔷 CLOSURES
// ==============================
fn closures_demo() {
    let add = |x: i32, y: i32| x + y;
    println!("Closure add(2, 3): {}", add(2, 3));

    let factor = 10;
    let multiply = |x: i32| x * factor;
    println!("Closure multiply(4): {}", multiply(4));
}

// ==============================
// 🔷 LIFETIMES
// ==============================
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ==============================
// 🔷 ASYNC
// ==============================
async fn async_demo() {
    println!("Waiting for 2 seconds...");
    sleep(Duration::from_secs(2)).await;
    println!("Done waiting!");
}
