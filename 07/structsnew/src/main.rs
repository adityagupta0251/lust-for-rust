/*
fn main() {
    println!("Hello, world!");
}
*/
// A User Struct definition
/*
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
*/


// Ex :
/*
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: Striing::from("someone@example.com"),
        sign_in_count: 1,
    };
}
*/


/*
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        
    };

    user1.email = String::from("anotheremail@example.com");
    
}
*/

/*
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

*/

/*
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
*/


// Using the Field Init Shorthand
/*
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
*/


// Creating Instances with Struct Update Syntax
/*
fn main() {

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String:;from("another@example.com"),
        sign_in_count: user1.sign_in_count,
        
    }
}
*/
/*
fn main() {
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1        
    }
}
*/

/*
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
}
*/

// Unit-like Structs
/*
struct AlwaysEqual;


fn main() {
    let subject = AlwaysEqual;
    
}
*/


/*
fn main() {
     let width = 20;
     let height = 50;
     println!(
         "The are of the rectangle is {} square pixels.",
         area(width , height)
         
     );
}


fn area (width: u32, height: u32) -> u32 {
    width * height
}
*/


// Refactoring with Tuples
/*
fn main() {
    let rect1 = (30, 50);
    println!(
        "the Area of the rectangle is {} square pixels.",
        area(rect1)
    );
    
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
    
}
*/


/*
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
        
    };
    
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/


// Adding Functionality with Derived Traits

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {rect1:?}");
}

// Note : To debug this we need to use Attributes by rust as following :
// #[derive(Debug)]


