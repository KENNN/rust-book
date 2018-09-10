fn main() {
    // let mut s = String::new(); // new string
    let data = "initial contents";
    let s = data.to_string();
    // let s = "initial contents".to_string(); // equal to above
    // let s = String::from("initial contents"); // equal too
    println!("{:?}", s);
}
