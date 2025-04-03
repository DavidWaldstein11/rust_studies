// variable lecture
const TYPE_OF_DATA: i8 = 2;
static mut A_STATIC_VARIABLE: i8 = 3;


fn main() {
    // use the variable on "unsafe mode"
    unsafe {
      A_STATIC_VARIABLE = 4;
   
    println!("Constant: {}", TYPE_OF_DATA);
    println!("Static: {}", A_STATIC_VARIABLE);
    }
    show();
}

fn show() {
    println!("Constant: {}", TYPE_OF_DATA);
    println!("Static: {}", A_STATIC_VARIABLE);
}
