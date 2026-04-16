/*
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The Length of '{s1}' is {len}. ");
    
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
*/
// Reference just allows you to use those pointed value without those ownership

// Close picture
/*
let s1 = String::from("hello");
let len = calculate_length(&s1);

*/


/*
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}
*/
// Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the String is not dropped.

/*
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    println!("{r1}, {r2}");
    // Variables r1 and r2 will not be used after this point.
    let r3 = &mut s;
    println!("{r3}");
    
}
*/



// Dangling Reference

// Bad Practice

/*
fn main() {
    let reference_to_nothing = dangle();

    
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
*/


fn dangle() -> &String {
    let s = String::from("hello");

    &s // we return a referenvce to the String, s
}

