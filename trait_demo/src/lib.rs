use std::fmt::{Display, Debug};

pub trait Summary {
    // fn summarize(&self) -> String;
    // 默认实现
    fn summarize(&self) -> String {
        format!("Read more...")
    }
}

pub struct News {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for News {}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} say {}", self.username, self.content)
    }
}

// trait 作为函数参数
pub fn notify(item: impl Summary) {
    println!("notify: {}", item.summarize());
}

// trait bound 语法, impl Trait 是语法糖
pub fn notify_bound<T: Summary>(item: T) {
    println!("notify: {}", item.summarize());
}

// 实现多个 Trait
pub fn notify_bound_m<T: Summary + Display>(item: T) {
    println!("notify: {}", item.summarize());
}

// 多个范型
pub fn notify_bound_2<T, U>(item1: T, item2: U) -> String
    where T: Summary + Display, U: Clone + Debug {
    format!("notify: {}, {}", item1.summarize(), item2.summarize)
}
