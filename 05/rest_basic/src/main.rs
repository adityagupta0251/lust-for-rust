/*
fn main() {
    println!("Hello, world!");
}
*/

// This is single line comment
/*

This is multi-line comment
*/

fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
            
        }
    };
    println!("The result is {result}");
}




