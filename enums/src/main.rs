struct Ipv4Addr {
    address: String,
}
struct Ipv6Addr {
    address: String,
}

impl Ipv4Addr {
    fn new(address: String) -> Ipv4Addr {
        Ipv4Addr { address }
    }
}

impl Ipv6Addr {
    fn new(address: String) -> Ipv6Addr {
        Ipv6Addr { address }
    }
}

fn main() {
    let home = Ipv4Addr::new(String::from("127.0.0.1"));
    let loopback = Ipv6Addr::new(String::from("::1"));

    println!("Home: {}", home.address);
    println!("Loopback: {}", loopback.address);

    let number: i32 = 5;
    let some_number: Option<i32> = Some(5);
    let absent_number: Option<i32> = None;

    let sum = number + some_number.unwrap() + absent_number.unwrap();

    println!("{}", &sum)
}
