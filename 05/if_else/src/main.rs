// If Statement

/* fn main() {
    println!("Hello, world!");

    let number = 3;
    if number < 5 {
        println!("Condition was true");
        
    } else {
        println!("condition was false");
    }
}

*/



// Handling Multiple conditions with else if
/*
fn main(){
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
        
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
        
    }else {
        println!("number isn't divisible by any of 4, 3, or 2");
    }
}
*/


fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    
}


// Bad practice
/*

fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}





*/


