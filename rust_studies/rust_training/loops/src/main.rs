// examples of loops in Rust

fn main() {
    println!("== loop (infinite with break) ==");
    loop_example();

    println!("\n== while loop ==");
    while_example();

    println!("\n== for loop over range ==");
    for_example();

    println!("\n== for loop over array ==");
    for_array_example();
}

// Basic loop (infinite loop with break)
fn loop_example() {
    let mut count = 0;

    loop {
        println!("Count: {}", count);
        count += 1;

        if count >= 5 {
            println!("Breaking out of the loop!");
            break;
        }
    }
}

// while loop example
fn while_example() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("Liftoff 🚀");
}

// for loop over a range
fn for_example() {
    for i in 1..5 {
        println!("i is {}", i);
    }

    // Including the upper bound
    for i in 1..=5 {
        println!("Now including 5: {}", i);
    }
}

// for loop over an array
fn for_array_example() {
    let names = ["Alice", "Bob", "Carol"];

    for name in names.iter() {
        println!("Hello, {}", name);
    }
}
