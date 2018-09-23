use std::net::IpAddr;

fn main() {
    // We know 127.0.0.1 is valid IP Addr but a conpiler don't.
    // The compiler requires use of Result type.
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}
