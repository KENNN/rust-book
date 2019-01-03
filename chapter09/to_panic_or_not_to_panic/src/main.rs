use std::net::IpAddr;

fn main() {
    // We know 127.0.0.1 is valid IP Addr but a conpiler don't.
    // The compiler requires use of Result type.
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    // Guidelines for Error Handling
    let guess = String::from("111");
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("not i32")
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
    }

    let g = Guess::new(111);
    println!("{:?}", g);
}

#[derive(Debug)]
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}