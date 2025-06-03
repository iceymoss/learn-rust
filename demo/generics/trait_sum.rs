
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;
}

struct Post {

}

impl Summary for Post {}

fn main() {
    let post = Post{};
    let summary = post.summarize();
    println!("{}", summary);
}