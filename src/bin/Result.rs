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
    fn verify_ip_address(&self) -> Result<String, String> {
        match self {
            IpAddr::V4(a, b, c, d) => println!("This is IPV4"),
            IpAddr::V6(a) => println!("This is IPV6"),
            IpAddr::Other(a) => match a {
                Some(b) => println!("This is some other IP"),
                None => return Err("Something went Wrong".to_owned()),
            },
        }
        return Ok("Ip Verified Successfully".to_owned());
    }

    fn check_ip(&self) -> Result<(), String> {
        let result = self.verify_ip_address()?;
        println!("{}", result);
        return Ok(());
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let office_ip = IpAddr::Other(Some(OtherIPType::Office));
    let unknown_ip = IpAddr::Other(None);

    match &home {
        IpAddr::V4(a, b, c, d) => println!("IPv4 address: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6 address: {}", addr),
        _ => println!("Other IP"),
    }

    let err = home.check_ip();
    let err2 = loopback.check_ip();
    let err3 = office_ip.check_ip();
    let err4 = unknown_ip.check_ip();

    println!("{:?}", err);
    println!("{:?}", err2);
    println!("{:?}", err3);
    println!("{:?}", err4);
}
