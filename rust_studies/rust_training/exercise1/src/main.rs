fn main() {
   /*
     Given that I have a year of birth, and I subtract it from the current year,
     Then I should get the person's age.
    */
    let name: &str = "John";

    let year_of_birth: u16 = 1990;
    let birth_month: u16 = 1;
    let birth_day: u16 = 31;

    let current_year: u16 = 2025;
    let current_month: u16 = 1;
    let current_day: u16 = 1;

    let mut age: i16 = current_year - year_of_birth;

    if birth_month > current_month {
      age -= 1;
    }
    else if birth_day > current_day {
      age -= 1;
    }

    println!("The age of {} calculated to the year of {} is {}", name, year_of_birth, age);
}

fn conditional_examples() {
  let number = 15;

  // Basic if statement
  if number > 10 {
      println!("{} is greater than 10", number);
  }

  // if-else statement
  if number % 2 == 0 {
      println!("{} is even", number);
  } else {
      println!("{} is odd", number);
  }

  // if-else if-else chain
  if number < 0 {
      println!("{} is negative", number);
  } else if number == 0 {
      println!("The number is zero");
  } else {
      println!("{} is positive", number);
  }

  // Using if as an expression to assign a value
  let is_even = if number % 2 == 0 { true } else { false };
  println!("Is the number even? {}", is_even);


// The "switch case" in rust

fn match_example() {
  let number = 3;

  match number {
      1 => println!("One"),
      2 => println!("Two"),
      3 => println!("Three"),
      4 | 5 | 6 => println!("Four, Five or Six"),
      7..=10 => println!("Between Seven and Ten"),
      _ => println!("Something else"),
  }

  let day = "Tuesday";

  match day {
      "Monday" => println!("Start of the work week"),
      "Friday" => println!("Almost the weekend!"),
      "Saturday" | "Sunday" => println!("Weekend!"),
      _ => println!("Just another weekday"),
  }
}
