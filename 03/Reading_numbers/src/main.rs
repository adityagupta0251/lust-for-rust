use std::io;
fn main() {
    println!("Enter a number");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("That wasn't a valid number!");
            return;
        }
        
    };
    println!("Your number plus ten is: {}", number + 10);
    
}
