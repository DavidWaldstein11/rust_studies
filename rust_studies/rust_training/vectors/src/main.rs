fn main() {
    // 📦 1. Creating Vectors
    let mut numbers = vec![1, 2, 3];
    println!("1. Initial vector: {:?}", numbers);

    // ➕ 2. Adding Elements
    numbers.push(4);
    numbers.push(5);
    println!("2. After pushing: {:?}", numbers);

    // ➖ 3. Removing Elements
    numbers.pop(); // removes the last element
    println!("3. After pop: {:?}", numbers);

    // 🧪 4. Accessing Elements
    println!("4. First: {}", numbers[0]);
    match numbers.get(10) {
        Some(val) => println!("Found: {}", val),
        None => println!("No value at index 10"),
    }

    // ✍️ 5. Modifying Elements
    numbers[1] = 99;
    println!("5. Modified vector: {:?}", numbers);

    // 🔄 6. Iterating Over Vectors
    println!("6. Loop with values:");
    for value in &numbers {
        println!("  - {}", value);
    }

    // 🎯 7. With Index
    println!("7. With index:");
    for (i, val) in numbers.iter().enumerate() {
        println!("  {}: {}", i, val);
    }

    // 🧪 8. Vector Length
    println!("8. Length: {}", numbers.len());

    // ✂️ 9. Slicing Vectors
    let slice = &numbers[1..];
    println!("9. Slice: {:?}", slice);

    // 📊 10. Finding Max/Min
    let max = numbers.iter().max().unwrap();
    let min = numbers.iter().min().unwrap();
    println!("10. Max: {}, Min: {}", max, min);

    // 📚 11. Vector of Strings
    let mut languages = vec!["Rust", "Go", "Python"];
    languages.push("TypeScript");
    println!("11. Languages: {:?}", languages);

    // ❌ 12. Clearing a Vector
    languages.clear();
    println!("12. After clear: {:?}", languages);
}
