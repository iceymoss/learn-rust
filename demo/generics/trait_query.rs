
trait Summary {
    fn summarize(&self) -> String;
}

struct Post {
    content: String,
    author: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.content)
    }
}

// 特性作为入参
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 特性作为入参
fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 特性作为返回值
fn returns_summarizable() -> impl Summary {
        Post {
        author: String::from("sunface"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        )
    }
}

fn main() {
    let post = Post{
        content: String::from("Hello, world"),
        author: String::from("Ubuntu"),
    };

    notify1(&post);

    notify(&returns_summarizable());
}