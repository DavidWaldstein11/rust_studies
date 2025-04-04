fn main() {
   /*
     Given that I have a year of birth, and I subtract it from the current year,
     Then I should get the person's age.
    */
    let name: &str = "John";

    let year_of_birth: u16 = 1990;
    let birth_month: u16 = 12;
    let current_year: u16 = 2025;

    let age: i16 = current_year - year_of_birth;

    println!("The age of {} calculated to the year of {} is {}", name, year_of_birth, age);
}
