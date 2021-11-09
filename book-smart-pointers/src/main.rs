use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    // box_demo();
    // my_box_demo();
    // drop_demo();
    rc_demo();
}

// Box 智能指针的基本使用
fn box_demo() {
    Cons(1, Box::new(
        Cons(2, Box::new(
            Cons(3, Box::new(Nil))))
    ));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 使用自定义 MyBox
fn my_box_demo() {
    let x = 12;
    let y = MyBox::new(x);

    assert_eq!(x, 12);
    assert_eq!(*y, 12);
    // 相当于
    assert_eq!(*y.deref(), 12);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

// 实现 Drop Trait 运行清理代码
fn drop_demo() {
    let _a = CustomSmartPointer { data: String::from("hello a") };
    // 可以手动使用 drop 方法清理代码
    drop(_a);
    let _b = CustomSmartPointer { data: String::from("hello b") };
    println!("CustomSmartPointers created");
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

// Rc<T> 引用计数智能指针
fn rc_demo() {
    let a = Rc::new(
        RcList::Cons(5, Rc::new(
            RcList::Cons(10, Rc::new(RcList::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcList::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcList::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// RcCell<T>

