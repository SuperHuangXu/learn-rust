use std::convert::{TryFrom, TryInto};

// 自定义类型转换
#[derive(Debug)]
struct Number {
    value: i32,
}

// 实现 From trait
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

struct Circle {
    radius: i32,
}

// ToString and FromStr
// 应该直接实现 `fmt::Display` trait, 它会自动提供 `ToString`，并且还可以打印类型
impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("{}", self.radius)
    }
}

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("Hello, {}", my_string);

    // Form
    let num = Number::from(13);
    println!("my number is {:?}", num);
    // Into，只要实现了 Form trait，则自动就有 Into.
    let int = 5;
    let num: Number = int.into();
    println!("my number is {:?}", num);
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(9), Err(()));
    // TryInto
    let res: Result<EvenNumber, ()> = 8_i32.try_into();
    assert_eq!(res, Ok(EvenNumber(8)));
    let res: Result<EvenNumber, ()> = 9_i32.try_into();
    assert_eq!(res, Err(()));

    let circle = Circle { radius: 12 };
    println!("{}", circle.to_string());

    // 解析字符串
    // 标准方法是使用 `parse` 函数，或者用"涡轮鱼"语法( turbo fish, <> )
    // 只要对目标类型实现了 `FromStr` trait,就可以用 `parse` 把字符串转换成目标类型
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    println!("{}", parsed + turbo_parsed);
}
