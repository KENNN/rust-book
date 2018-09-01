fn main() {
    let coin = Coin::Penny;
    value_in_cents(coin);

    // Patterns that Bind to Values
    let coin2 = Coin::Quater(UsState::Alabama);
    value_in_cents(coin2);

    // Matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
    println!("{:?}", none);

    // The _ Placeholder
    let some_u8_value  = 0u8;
    match some_u8_value {
        1 => println!("{:?}", "one"),
        3 => println!("{:?}", "three"),
        5 => println!("{:?}", "five"),
        7 => println!("{:?}", "seven"),
        _ => println!("{:?}", ()),
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("{:?}", "Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}