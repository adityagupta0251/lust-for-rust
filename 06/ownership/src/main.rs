/*
fn main() {
    let s = "hello";
    println!("Hello, world!");


    // the scope of s is now over , and s is no longer valid 
}
*/

// The String Type
/*
fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("{s}");

    
}
*/

/*
fn main() {
    let x = 5;
    let y = x;
    println!("The value of y is: {y}");

    
}
*/
// Assigning the integer value of variable x to y
/*
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s2} ,world!");
       
}
*/

// Either if you try the following example :
/*

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");
*/

/*
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:16
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{s1}, world!");
  |                ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error

*/

/*
fn main() {
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");
    
}
*/

// Without deeply copying the heap data of String, not just the stack data => Use clone
/*
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

}
*/

fn main() {
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
    
}
