fn main() {
    // what is string?
    // let mut s = String::new(); // new string
    let data = "initial contents";
    let s = data.to_string();
    // let s = "initial contents".to_string(); // equal to above
    // let s = String::from("initial contents"); // equal too
    println!("{:?}", s);

    let hello = vec![
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שָׁלוֹם"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];
    for h in hello {
        println!("{:?}", h);
    }


}
