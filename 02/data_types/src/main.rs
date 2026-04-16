fn main() {
    println!("Hello, world!");

    // with type annotations
    // let guess: u32 = "42",parse().expect("Not a number!");

    // Scalar types:
    // A scalar types represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans , and characters.
    // Above looks as same as probably all programming languages.

    // Integer Types:
    // 8-bit Signed:i8 Unsigned:u8
    // 16-bit Signed:i16 Unsigned:u16
    // 32-bit Signed:i32 Unsigned:u32
    // 64-bit Signed:i64 Unsigned:u64
    // 128-bit Signed:i128 Unsigned:u128
    // Architechture-dependent Signed:isize Unsigned:usize


    // Each Signed variant can store numbers from -(2^n-1) to 2^n-1 - 1
    // eg : i8 = -(2^7) to 2^7 - 1 which equals to -128 to 127


    // Number Literals Example
    // Decimal 98_222
    // Hex 0xff
    // Octal 0o77
    // Binary 0b1111_0000
    // Byte(u8 only) b'A'


    // Floating-Point Types
    let x = 2.0; //f64
    let y: f32 = 3.0; //f32

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");


    //Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -21 / 2;
    let remainder = 43 % 5;

    println!("The sum is: {sum}");
    println!("The difference is: {difference}");
    println!("The product is: {product}");
    println!("The quotient is: {quotient}");
    println!("The truncated is: {truncated}");
    println!("The remainder is: {remainder}");


    // The Boolean type
    let boo = true;
    let boof: bool = false;

    // Concatiniton using placeholder
    println!("{} {}", boo, boof);

    // Using Captured variables
    // println!("first boolean is {boo}!");
    
    // Using the + Operator
    // let combined = boo + boof;
    // println!("{}", combined);
    
    // Using format! macro
    // let s1 = "Hello";
    // let s2 = "World";
    // let s3 = format!("{s1}, {s2}!");
    // println!("{}, s2");

    // Using the concat! Macro for literals
    // println!(concat!("Hello", " ", "World"));


    // Note : Only + Operator uses "Consumes first type of Ownership" this is memory efficient (reuses better)
    // Note : concat! method doesn't uses any ownership type like. it is hardcoded constants.
    // Only println!("{a} {b}") uses as in modern way! it uses Borrows like ownership
    // & lastly println!("{} {}", a,b); use as in simple output for..


    // The Character Type
    // let c = 'z';
    // let z: char = 'Z'; //with explicit type annotation
    // let heart_eyed_cat = '😻'; 



    // The Compound Types
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
    
    // 1.1 The Tuple Type
    let tup: (i32 , f64, u8) = (500, 6.4, 1);
    let (zz, yy , xx) = tup;
    println!("The value of yy is: {yy}");
    
    // Also accesing through period way:
    // let x: (i32 , f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.3;


    // The Array Type:
    // let a = [1,2,3,4,5];
    // let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "..."];
    //
    // let a: [i32; 5] = [1,2,3,4,5];

    // Initialising through same value with your given times
    // let a = [3;5] same as : let a = [3, 3, 3, 3, 3]; 
    // Above is just like : let x = [a_number;that_time];
    
    
    // Array ELement Access
    // let a = [1, 2, 3, 4, 5];
    // let first = a[0];
    // let second = a[1];
    // let third = a[3];
    // let fourth = a[4];



    // Taking input from user
    /*
    use std::io; // Import the standard I/O library

fn main() {
    println!("Please enter some text:");

    // 1. Create a mutable String buffer to store the input
    let mut input = String::new();

    // 2. Read the input from the standard input stream
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); // 3. Handle potential errors

    // 4. Print the input (it includes the newline character by default)
    println!("You wrote: {}", input);
}






    */
    // Note : the u* series (Unsigned) annotations only used for whole or non-negative integer rather i* series (Signed integers) uses negative & positive both integers.
     
}
