fn main() {
    /* dangling refference
    {
        let r;                  // ---------+--'a
        {                       //          |
            let x = 5;          // --+'b    |
            r = &x;             //   |      |
        }                       // --+      |
        println!("r: {}", r);   //          |
    }*/                         // ---------+

    // The Borrow Cheacker
    {
        let x = 5;              // ---------+--'b
                                //          |
        let r = & x;            // --+--'a  |
                                //   |      |
        println!("r: {}", r);   //   |      |
                                // --+      |
    }                           // ---------+

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y:&str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}