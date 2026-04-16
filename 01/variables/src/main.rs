fn main() {
    // let x = 5; bad practice
    let mut x = 5;
    
    println!("The Value of x is: {x}");
    x = 6;
    println!("Now , The Value of x is: {x}");
    println!("Hello, world!");


    // declarimg constants
    // now we are just using data types : will cover all in next practice
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value is: {}", THREE_HOURS_IN_SECONDS);
    // to check the type of variables
    // use std::any::type_name;
    // fn type_of<T>(_: &T) -> &'static str {
    //     type_name::<T>()
    // }
    // fn main() {
    //
    //    let x = 42.0;
    //    println!("x is {}", type_of(&x));
    // 
    // }


    // Shadowing : scope ends & valid under curvly braces

    let sh = 8;
    let sh = sh + 1;
    {
        let sh = sh * 2;
        println!("The value of sh in the inner scope is: {sh} ");
        
    }
    println!("The value of sh is: {sh}");

    // difference between shawdowing & mutability
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Length is: {spaces}");
    // Mutable:
    // let mut space = "   ";
    // space = space.len();
    // $ cargo run
    //   Compiling variables v0.1.0 (file:///projects/variables)
    // error[E0308]: mismatched types
    // --> src/main.rs:3:14
    //   |
    // 2 |     let mut spaces = "   ";
    //   |                      ----- expected due to this value
    // 3 |     spaces = spaces.len();
    //   |              ^^^^^^^^^^^^ expected `&str`, found `usize`

    // For more information about this error, try `rustc --explain E0308`.
    // error: could not compile `variables` (bin "variables") due to 1 previous error

}
