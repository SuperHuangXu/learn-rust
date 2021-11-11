use std::fmt::format;
use std::ops::{Add};
use std::option::Option;

fn main() {
    struct Sheep {
        naked: bool,
        name: &'static str,
    }

    trait Animal {
        // 静态方法签名；`Self` 表示实现者类型（implementor type）。
        fn new(name: &'static str) -> Self;
        // 实例方法签名；这些方法将返回一个字符串。
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;
        // trait 可以提供默认的方法定义。
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }
        fn shear(&mut self) {
            if self.is_naked() {
                println!("{} is already naked...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);
                self.naked = true;
            }
        }
    }

    impl Animal for Sheep {
        fn new(name: &'static str) -> Self {
            Sheep { name, naked: false }
        }
        fn name(&self) -> &'static str {
            self.name
        }
        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "bbb?"
            } else {
                "bbb!"
            }
        }
        fn talk(&self) {
            println!("{} pauses briefly... {}", self.name, self.noise())
        }
    }

    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();

    // 派生
    // 通过 #[derive] 属性，编译器能够提供某些 trait 的基本实现。
    // 比较 trait: Eq, PartialEq, Ord, PartialOrd
    // Clone, 用来从 &T 创建副本 T。
    // Copy，使类型具有 “复制语义”（copy semantics）而非 “移动语义”（move semantics）。
    // Hash，从 &T 计算哈希值（hash）。
    // Default, 创建数据类型的一个空实例。
    // Debug，使用 {:?} formatter 来格式化一个值。
    #[derive(Default, Debug)]
    struct Dog {
        name: String,
        age: u8,
    }
    let dog: Dog = Default::default();
    println!("{:?}", dog);

    // 使用 dyn 返回 trait
    struct A {}
    struct B {}
    trait Person {
        fn noise(&self) -> &'static str;
    }
    impl Person for A {
        fn noise(&self) -> &'static str {
            "aaoooooooo!"
        }
    }
    impl Person for B {
        fn noise(&self) -> &'static str {
            "bboooooooo!"
        }
    }
    fn random_person(random_number: f64) -> Box<dyn Person> {
        if random_number < 0.5 {
            Box::new(A {})
        } else {
            Box::new(B {})
        }
    }

    let animal = random_person(0.2);
    println!("{}", animal.noise());

    // 运算符重载
    struct Foo;
    impl Add<&str> for Foo {
        type Output = String;
        fn add(self, rhs: &str) -> Self::Output {
            String::from("hello") + rhs
        }
    }
    let foo = Foo {};
    println!("{}", foo + " world.");

    // Iterator
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let new_next = self.curr + self.next;
            self.curr = self.next;
            self.next = new_next;
            Some(self.curr)
        }
    }
    for x in (Fibonacci { curr: 1, next: 1 }).take(5) {
        println!("> {}", x);
    }

    // impl Trait
    fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item=i32> + 'a {
        numbers.iter()
            .filter(|x| x > &&0)
            .map(|x| x * 2)
    }
    let v = vec![1, 2, 3, -1, 0];
    let r = double_positives(&v);
    for ele in r {
        println!("{}", ele);
    }

    // Clone
    #[derive(Copy, Clone, Debug)]
    struct Nil;
    #[derive(Clone, Debug)]
    struct Pair(Box<i32>, Box<i32>);

    let nil = Nil;
    // 复制 `Nil`，没有资源用于移动（move）
    let copied_nil = nil;
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);
    let moved_pair = pair;
    // pair 已经被移动
    // println!("original: {:?}", pair);
    println!("copy: {:?}", moved_pair);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);
    // 报错！`moved_pair` 已被销毁。
    //println!("copy: {:?}", moved_pair);
    // 由 .clone() 得来的结果仍然可用！
    println!("clone: {:?}", cloned_pair);

    // 父 trait
    trait Baba {
        fn name(&self) -> String;
    }
    trait Student: Baba {
        fn university(&self) -> String;
    }
    trait Programmer {
        fn fav_language(&self) -> String;
    }
    // 实现 CompSciStudent 需要你同时 impl 了两个父 trait。
    trait CompSciStudent: Student + Programmer {
        fn git_username(&self) -> String;
    }

    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!("My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
                student.name(),
                student.university(),
                student.fav_language(),
                student.git_username())
    }

    // 消除重叠 trait
    trait UsernameWidget {
        fn get(&self) -> String;
    }
    trait AgeWidget {
        fn get(&self) -> u8;
    }
    #[derive(Debug)]
    struct Form {
        username: String,
        age: u8,
    }
    impl UsernameWidget for Form {
        fn get(&self) -> String {
            self.username.clone()
        }
    }
    impl AgeWidget for Form {
        fn get(&self) -> u8 {
            self.age
        }
    }
    let form = Form { username: "xm".to_owned(), age: 12 };
    // println!("{}", form.get());
    let un = <Form as UsernameWidget>::get(&form);
    println!("form username: {}", un);
    let age = AgeWidget::get(&form);
    println!("form age: {}", age);
}
