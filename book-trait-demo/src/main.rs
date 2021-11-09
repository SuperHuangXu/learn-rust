use trait_demo::{Weibo, Summary, notify};

fn main() {
    let w = Weibo {
        username: String::from("123456"),
        content: String::from("内容"),
    };

    // println!("{}", w.summarize());
    notify(w);
}
