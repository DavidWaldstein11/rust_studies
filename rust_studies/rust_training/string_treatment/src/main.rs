fn main() {
    // 🧵 1. Creating Strings
    let s1 = String::new();
    let s2 = String::from("Hello");
    let s3 = "World".to_string();
    println!("1. {} {}", s2, s3);

    // ➕ 2. Appending and Concatenation
    let mut s = String::from("Hello");
    s.push(' ');
    s.push_str("world!");
    let name = "David";
    let msg = format!("2. Welcome, {}!", name);
    println!("{}", msg);

    // 📏 3. Length and Iteration
    let s = String::from("Olá");
    println!("3. Length in bytes: {}", s.len());

    println!("   Characters:");
    for c in s.chars() {
        println!("   {}", c);
    }

    println!("   Bytes:");
    for b in s.bytes() {
        println!("   {}", b);
    }

    // ✂️ 4. Slicing Strings
    let s = String::from("Hello world");
    let hello = &s[0..5];
    println!("4. Slice: {}", hello);

    // 🔍 5. Searching and Replacing
    let text = String::from("hello world");
    println!("5. Contains 'world'? {}", text.contains("world"));
    println!("   Starts with 'he'? {}", text.starts_with("he"));
    let replaced = text.replace("world", "Rust");
    println!("   Replaced: {}", replaced);

    // 🧼 6. Trimming and Splitting
    let raw = "  rustacean  ";
    println!("6. Trimmed: '{}'", raw.trim());

    let csv = "apple,banana,grape";
    println!("   Split CSV:");
    for fruit in csv.split(',') {
        println!("   - {}", fruit);
    }
}
