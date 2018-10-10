use std::fmt::Display;

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
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    /* error
    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    */

    // Lifetime Annotation in Struct Definition
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");

    let i = ImportantExcerpt { part: first_sentence };
    println!("{:?}", i);

    // Lifetime Annotation in Method Definitions
    println!("{}", i.announce_and_return_part("read the first sentence"));

    // Static Lifetime
    let s : &'static str = "I have a static lifetime";

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together
    let x = "abc";
    let y = "abcd";
    let ann = "Fafafa!";
    let longer = longest_with_an_announcement(x, y, ann);
    println!("{:?}", longer);
}

fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2<'a> (x: &'a str, y: &str) -> &'a str {
    x
}


/* error
fn longest3<'a> (x: &str, y:&str) -> 'a str {
    let result = String::from("really long string");
    result.as_str()
}
*/

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

