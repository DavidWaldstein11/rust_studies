fn main() {
    println!("== Arithmetic Operators ==");
    arithmetic_operators();

    println!("\n== Comparison Operators ==");
    comparison_operators();

    println!("\n== Logical Operators ==");
    logical_operators();

    println!("\n== Assignment Operators ==");
    assignment_operators();

    println!("\n== Bitwise Operators ==");
    bitwise_operators();

    println!("\n== Range and Membership ==");
    range_and_membership();
}

// ==============================
// 🔹 Arithmetic
// ==============================
fn arithmetic_operators() {
    let a = 10;
    let b = 3;

    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b); // Integer division
    println!("a % b = {}", a % b);

    let c = 10.0;
    let d = 3.0;
    println!("c / d = {}", c / d); // Float division
}

// ==============================
// 🔹 Comparison
// ==============================
fn comparison_operators() {
    let x = 5;
    let y = 8;

    println!("x == y: {}", x == y);
    println!("x != y: {}", x != y);
    println!("x > y: {}", x > y);
    println!("x < y: {}", x < y);
    println!("x >= y: {}", x >= y);
    println!("x <= y: {}", x <= y);
}

// ==============================
// 🔹 Logical
// ==============================
fn logical_operators() {
    let a = true;
    let b = false;

    println!("a && b = {}", a && b);
    println!("a || b = {}", a || b);
    println!("!a = {}", !a);
}

// ==============================
// 🔹 Assignment and Compound
// ==============================
fn assignment_operators() {
    let mut x = 5;

    println!("Initial x: {}", x);
    x += 3;
    println!("x += 3 → {}", x);
    x -= 2;
    println!("x -= 2 → {}", x);
    x *= 4;
    println!("x *= 4 → {}", x);
    x /= 2;
    println!("x /= 2 → {}", x);
    x %= 3;
    println!("x %= 3 → {}", x);
}

// ==============================
// 🔹 Bitwise
// ==============================
fn bitwise_operators() {
    let a: u8 = 0b1010; // 10
    let b: u8 = 0b1100; // 12

    println!("a & b = {:04b}", a & b); // AND
    println!("a | b = {:04b}", a | b); // OR
    println!("a ^ b = {:04b}", a ^ b); // XOR
    println!("!a = {:08b}", !a);       // NOT
    println!("a << 1 = {:04b}", a << 1); // Left shift
    println!("b >> 2 = {:04b}", b >> 2); // Right shift
}

// ==============================
// 🔹 Ranges and Membership
// ==============================
fn range_and_membership() {
    for i in 1..5 {
        print!("{} ", i); // 1 to 4
    }
    println!("→ (1..5)");

    for i in 1..=5 {
        print!("{} ", i); // 1 to 5
    }
    println!("→ (1..=5)");

    let x = 3;
    let is_in_range = (1..=5).contains(&x);
    println!("Is 3 in 1..=5? {}", is_in_range);
}
