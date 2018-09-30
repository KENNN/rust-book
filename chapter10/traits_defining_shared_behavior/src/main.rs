mod lib;
use lib::Summary;
use lib::Tweet;
use lib::NewArticle;

fn main() {
    let tweet = Tweet {
        username: String::from("house_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // Default Implementations
    let article = NewArticle {
        headline: String::from("Penguins win the Stranley Cup Championship"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());
}
