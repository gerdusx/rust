// Enum with normal variants
enum State {
    Open,
    Closed,
}

// Enum with struct-like variants
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
struct CustomIpAddr {
    // --snip--
}

// Enum with different types in variants
#[derive(Debug)]
enum IpAddrV2 {
    V4(u8, u8, u8, u8),  // Tuple struct variant
    V6(String),          // String variant
    Custom(CustomIpAddr),// Struct variant
}

// Implementing methods on enums
impl IpAddrV2 {
    fn reset(&mut self) {
        *self = IpAddrV2::V6(String::from("0.0.0.0"));
    }
}

fn main() {
    // Creating enum instances
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // Using enum with methods
    let mut addr = IpAddrV2::V6(String::from("10.10.10.2"));
    println!("addr before: {:?}", addr); // addr before: V6("10.10.10.2")
    addr.reset();
    println!("addr after: {:?}", addr); // addr after: V6("0.0.0.0")
}
