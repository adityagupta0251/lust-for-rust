use std::io;


fn main() {
    println!("Please enter some text:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); // Handles potential errors


    println!("You Wrote: {}", input);

    
}
