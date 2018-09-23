use std::net::IpAddr;

fn main() {
    // We know 127.0.0.1 is valid IP Addr but a conpiler don't.
    // The compiler requires use of Result type.
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    let guess = String::from("111");
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("not i32")
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        return;
    }
}