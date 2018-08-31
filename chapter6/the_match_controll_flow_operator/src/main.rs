fn main() {
    let coin = Coin::Penny;
    value_in_cents(coin);
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("{:?}", "Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater => 25,
    }
}

