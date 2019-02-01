fn main() {
    let some_option_value = None;
    if let Some(x) = some_option_value {
        println!("{:?}", x);
    }

    if let x = 5 {
        println!("{:?}", x);
    }
}
