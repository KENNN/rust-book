fn main() {
    // Enum Values
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}", four);
    println!("{:?}", six);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("::1"),
    };

    println!("{:?}", home);
    println!("{:?}", loopback);

    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    let home = IpAddrEnum2::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum2::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    let home = IpAddrEnum3::V4(Ipv4Addr{});
    let loopback = IpAddrEnum3::V6(Ipv6Addr{});

    println!("{:?}", home);
    println!("{:?}", loopback);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum IpAddrEnum2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct Ipv4Addr {}

#[derive(Debug)]
struct Ipv6Addr {}

#[derive(Debug)]
enum IpAddrEnum3 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn route(ip_type: IpAddrKind) {
    println!("{:?}", ip_type);
}
