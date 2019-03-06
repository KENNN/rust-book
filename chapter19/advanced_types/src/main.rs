type Kilometers = i32;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let guess = "5";
    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }

    print!("forever ");

    loop {
        print!("and ever ");
        break
    }
}
