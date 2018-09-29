mod lib;
use lib::Summary;
use lib::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("house_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
