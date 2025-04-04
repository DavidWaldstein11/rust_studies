// Single-line comments start with two slashes

/*
Multi-line comments are written like this.
They can span multiple lines and are useful for longer explanations.
*/

fn main() {
    all_data_types();
}

fn all_data_types() {
    // Integer types
    let int8: i8 = -128; // 8-bit signed integer
    let uint8: u8 = 255; // 8-bit unsigned integer
    let int16: i16 = -32768; // 16-bit signed integer
    let uint16: u16 = 65535; // 16-bit unsigned integer
    let int32: i32 = -2147483648; // 32-bit signed integer
    let uint32: u32 = 4294967295; // 32-bit unsigned integer
    let int64: i64 = -9223372036854775808; // 64-bit signed integer
    let uint64: u64 = 18446744073709551615; // 64-bit unsigned integer
    let int128: i128 = -170141183460469231731687303715884105728; // 128-bit signed integer
    let uint128: u128 = 340282366920938463463374607431768211455; // 128-bit unsigned integer
    let arch_int: isize = -1000; // Architecture-dependent signed integer
    let arch_uint: usize = 1000; // Architecture-dependent unsigned integer

    // Floating point types
    let float32: f32 = 3.14; // 32-bit floating point number
    let float64: f64 = 3.141592653589793; // 64-bit floating point number

    // Boolean type
    let boolean: bool = true; // Boolean value (true or false)

    // Character type
    let character: char = 'A'; // A single Unicode character

    // String types
    let string_literal: &str = "Hello, Rust!"; // String slice (immutable)
    let mut owned_string: String = String::from("Hello, world!"); // Growable heap-allocated string

    // Array type
    let array: [i32; 3] = [1, 2, 3]; // Fixed-size array

    // Tuple type
    let tuple: (i32, f64, char) = (42, 3.14, 'Z'); // Tuple with mixed types

    // Slice type
    let slice: &[i32] = &array[1..3]; // Slice of an array

    // Vector type (dynamic array)
    let mut vector: Vec<i32> = vec![10, 20, 30]; // Growable list
    vector.push(40);

    // References
    let reference: &i32 = &int32; // Immutable reference
    let mut mutable_variable: i32 = 10;
    let mutable_reference: &mut i32 = &mut mutable_variable; // Mutable reference

    // Option type (nullable values)
    let some_value: Option<i32> = Some(10); // Some variant
    let no_value: Option<i32> = None; // None variant

    // Result type (error handling)
    let success: Result<i32, &str> = Ok(100); // Ok variant
    let failure: Result<i32, &str> = Err("Something went wrong"); // Err variant

    // Struct type
    struct Person {
        name: String,
        age: u8,
    }
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Enum type
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    let direction = Direction::Up;

    // Printing some values
    println!("Integer: {}, Floating Point: {}, Boolean: {}, Char: {}", int32, float64, boolean, character);
    println!("String Literal: {}, Owned String: {}", string_literal, owned_string);
    println!("Array: {:?}, Tuple: {:?}, Slice: {:?}", array, tuple, slice);
    println!("Vector: {:?}", vector);
    println!("Reference: {}, Mutable Reference: {}", reference, mutable_reference);
    println!("Option Some: {:?}, Option None: {:?}", some_value, no_value);
    println!("Result Success: {:?}, Result Failure: {:?}", success, failure);
    println!("Struct Person: Name = {}, Age = {}", person.name, person.age);
    println!("Enum Direction: {:?}", direction);
}

