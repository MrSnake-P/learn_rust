use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("l ate an egg");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("l ate an egg", post.content());
}