enum OtherIPType {
    Local,
    Home,
    Office,
}
enum IpAddr {
    V4(u8, u8, u8, u8), // Tuple-like variant for IPv4 addresses
    V6(String),         // Tuple-like variant for IPv6 addresses
    Other(Option<OtherIPType>),
}

impl IpAddr {
    fn print_ip_addr(&self) {
        match self {
            IpAddr::V4(a, b, c, d) => println!("IPv4 address: {}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(addr) => println!("IPv6 address: {}", addr),
            IpAddr::Other(Some(other)) => match other {
                OtherIPType::Home => println!("Home Other Type"),
                _ => println!("Home Other Type"),
            },
            IpAddr::Other(None) => println!("Other IP"),
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let office_ip = IpAddr::Other(Some(OtherIPType::Office));
    let other_ip = IpAddr::Other(None);

    home.print_ip_addr();
    loopback.print_ip_addr();
    office_ip.print_ip_addr();
    other_ip.print_ip_addr();
}
