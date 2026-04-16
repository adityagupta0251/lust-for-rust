/*
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // Creating an Instance :
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
}

// Using as in Function's parameter
fn route(ip_kind: IpAddrKind) {
    
}

// either :
route(IpAddrKind::V4);
route(IpAddrKind::V6);
*/

// More Advantages :
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind::v4,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
    
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
    
};


enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
    
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));




