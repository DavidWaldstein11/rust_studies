use std::io;

fn main() {
  loop {
     println!("Digite uma opção abaixo");
     println!(r#"
       Option 1
       Option 2
       Option 3
     "#);

     let mut str_option: String = String::new();
     io::stdin()
       .read_line(&mut str_option);
       .expect("Failed to read line");

    let option: i8 = option.trim().parse().expect("Please, type a number");

    match option {
        1 => println!("You chose option 1"),
        2 => println!("You chose option 2"),
        3 => println!("You chose option 3"),
        4 =>  break,
        _ => println!("Invalid option"),
    }
  }
}
