use trait_::NewsArticle;
use trait_::Summary;

fn main() {
    let article = NewsArticle {
        title: String::from("a"),
        description: String::from("b"),
    };

    println!("{}", article.summarize());
}
