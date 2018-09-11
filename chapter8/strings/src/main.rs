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

    // Updating  a String
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{:?}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("{:?}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("{:?}", s);

    //Appending to a String with push_str and push
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("{:?}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{:?}", s);
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{:?}", s);

    // Indexing into Strings
    let s1 = String::from("hello");
    // let h = s1[0]; error

    // Slicning Strings
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{:?}", s);
}
