


//Create an enum to represent ipv4 and ipv6 addresses
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    //A variable to store an IPv4 address
    let home = IpAddr::V4(String::from("127.0.0.!"));
    //A variable to store an IPv6 address
    let loopback = IpAddr::V6(String::from("::1"));
}
