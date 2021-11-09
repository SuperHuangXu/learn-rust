#[allow(dead_code)]
#[derive(Debug)]
enum Color {
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];
    // `iter` 在每次迭代中借用集合中的一个元素
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);
    // `into_iter` 会消耗集合，每次迭代中，集合中的数据本身会被提供
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // 下面这句话会报错
    // println!("names: {:?}", names);

    let mut names = vec!["Bob", "Frank", "Ferris"];
    // `iter_mut` 可变地( mutable ) 借用集合中的每个元素，从而允许集合被就地修改
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello"
        }
    }
    println!("names: {:?}", names);

    // match 匹配
    let number = 19;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One"),
        2 | 3 | 4 | 5 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special")
    }
    let boolean = true;
    // 表达式
    let binary = match boolean {
        false => 0,
        true => 1
    };
    assert_eq!(binary, 1);

    // 解构
    // 元祖
    let pair = (0, -2);
    match pair {
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("x is {} and last is `0`", x),
        _ => println!("It doesn't matter what they are")
    }
    // 枚举
    let color = Color::RGB(122, 17, 40);
    match color {
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
    }
    // 指针和引用
    // 解引用使用`*`，解构使用`&`/`ref`/`ref mut`
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    // 如果不想用`&`
    match *reference {
        val => println!("Got a value via destructuring: {:?}", val),
    }
    // rust 可以使用 `ref` 更改赋值行为，从而可以对具体值创建引用。
    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;
    match value {
        ref v => println!("Got a reference to a value: {:?}", v),
    }
    match mut_value {
        ref mut v => {
            // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
            *v += 10;
            println!("We added 10. `mut_value`: {:?}", v);
        }
    }

    // 结构体
    let foo = Foo {
        x: (1, 2),
        y: 3,
    };
    let Foo { x: (a, b), y } = foo;
    println!("a = {}, b = {},  y = {} ", a, b, y);
    // 重命名变量
    let Foo { x: i, y: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);
    // 忽略某些变量
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // 卫语句 （ guard ）
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("There are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    // 绑定
    fn age() -> u32 {
        15
    }
    match age() {
        0 => println!("I'm not born yet I guess"),
        // 使用 `@` 符号绑定变量到名称
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // 不符合上面的范围。返回结果
        n => println!("I'm an old person of age {:?}", n),
    }
    fn some_number() -> Option<u32> {
        Some(12)
    }
    match some_number() {
        Some(n @ 12) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }

    // if let
    let number = Some(7);
    if let Some(i) = number {
        println!("Matched {:?}!", i)
    } else {
        // 解构失败。切换到失败情形。
        println!("Didn't match a number. Let's go with a letter!");
    }
    // 枚举
    let color = Color::RGB(122, 17, 40);
    if let Color::RGB(122, 17, 40) = color {
        println!("{:?}", color);
    }

    // while let
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 10 {
            println!("Greater than 11, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}
