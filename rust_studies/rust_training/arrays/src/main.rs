fn main() {
    // 📦 1. Creating Arrays
    let numbers = [1, 2, 3, 4, 5];
    println!("1. First array: {:?}", numbers);

    // 📏 2. Fixed Length and Type
    let even: [i32; 4] = [2, 4, 6, 8]; // [type; size]
    println!("2. Even numbers: {:?}", even);

    // 🔁 3. Repeating Values
    let repeated = [0; 5]; // 5 zeros
    println!("3. Repeated zeros: {:?}", repeated);

    // 🎯 4. Accessing Elements
    println!("4. First number: {}", numbers[0]);
    println!("   Last number: {}", numbers[numbers.len() - 1]);

    // ⚠️ 5. Index Out of Bounds (will panic!)
    // println!("Invalid access: {}", numbers[10]); // Uncomment to see panic

    // ✍️ 6. Mutability
    let mut scores = [10, 20, 30];
    scores[1] = 99;
    println!("6. Modified scores: {:?}", scores);

    // 🔄 7. Iterating Over Arrays
    println!("7. Loop through array:");
    for (index, value) in numbers.iter().enumerate() {
        println!("   Index {}: Value {}", index, value);
    }

    // ✂️ 8. Slicing Arrays
    let slice = &numbers[1..4]; // [2, 3, 4]
    println!("8. Slice of numbers: {:?}", slice);

    // 📊 9. Finding Max/Min (manual example)
    let mut max = numbers[0];
    for &num in &numbers {
        if num > max {
            max = num;
        }
    }
    println!("9. Max value: {}", max);

    // 📚 10. Array of Strings
    let words = ["Rust", "is", "awesome"];
    for word in &words {
        println!("10. Word: {}", word);
    }
}
