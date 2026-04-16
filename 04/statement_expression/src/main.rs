// Bad practices

// fn main() {
//   ley y = 6;
// }


// fn main() {
//    ley x = (let y = 6);
// }
//
//
// Error will be :
/*
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found `let` statement
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: only supported directly in conditions of `if` and `while` expressions

warning: unnecessary parentheses around assigned value
 --> src/main.rs:2:13
  |
2 |     let x = (let y = 6);
  |             ^         ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
2 -     let x = (let y = 6);
2 +     let x = let y = 6;
  |

warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` (bin "functions") due to 1 previous error; 1 warning emitted



*/





// Good Practice
//fn main(){
//    let y = {
//        let x = 3;
//        x + 1
//        
//    };
//    println!("The value of y is: {y}");
//
//    
//}


// Functions with Return Values
fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    println!("The value of x is: {x}");

    
}
