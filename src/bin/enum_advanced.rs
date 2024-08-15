enum OtherIPType {
    Local,
    Home,
    Office,
}
enum IpAddr {
    V4(u8, u8, u8, u8), // Tuple-like variant for IPv4 addresses
    V6(String),         // Tuple-like variant for IPv6 addresses
    Other(OtherIPType),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let office_ip = IpAddr::Other(OtherIPType::Office);

    match home {
        IpAddr::V4(a, b, c, d) => println!("IPv4 address: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6 address: {}", addr),
        _ => println!("Other IP"),
    }
}
