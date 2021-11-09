mod linked_list;

use crate::linked_list::List;

// 元祖结构体
struct Pair(i32, f32);

// 带字段的结构体
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
// 结构体可以作为另一个结构体的字段
struct Rectangle {
    p1: Point,
    p2: Point,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted {}", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        }
    }
}

fn main() {
    let person = Person {
        name: String::from("xm"),
        age: 12,
    };
    println!("Hello, {:?}", person);
    // 使用结构体更新语法创建新的 person，这样可以用到之前的 person 的字段
    let new_person = Person {
        age: 24,
        ..person
    };
    println!("Hello, {:?}", new_person);

    let point = Point { x: 0.3, y: 0.4 };
    // 使用 `let` 绑定来解构 point
    let Point { x: my_x, y: my_y } = point;
    println!("x: {}, y: {}", my_x, my_y);

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 30 };
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(WebEvent::PageLoad);
    inspect(WebEvent::PageUnload);

    // 枚举
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
    println!("nil length: {}", List::new().len());

    // 常量
    // - `const` 不可变的值
    // - `static` 具有 'static 生命周期的值，可以是可变的变量
    static LANGUAGE: &'static str = "Rust";
    const THRESHOLD: i32 = 10;
}
