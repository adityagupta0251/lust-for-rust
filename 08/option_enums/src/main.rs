 fn main() {
    println!("Hello, world!");
    enum Option<T> {
        None,
        Some(T),
    }
    let some_number = Some(5);
    let some_char = Some('e');
    println!("The value of some_number is: {:?}", some_number);

    let absent_number: Option<i32> = None;
}



