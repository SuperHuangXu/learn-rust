use std::mem;
use std::ops::Deref;

struct Rectangle {
    x: f64,
    y: f64,
}

impl Rectangle {
    // `&self` 是 `self: &Self` 的语法糖，其中 `Self` 是方法调用者的类型。这里 `Self` == `Rectangle`
    fn area(&self) -> f64 {
        self.x * self.y
    }
    // 这个方法要求调用者是可变的
    // `&mut self` 为 `self: &mut Self` 的语法糖
    fn translate(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // 这个方法会 “消耗” 调用者的资源
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
        // `first` 和 `second` 离开作用域后释放
    }
}

fn main() {
    // 闭包
    // 闭包是匿名的，闭包表达式产生的类型就是 “闭包类型”，不属于引用类型。
    // 类型标注和函数的一样，不过类型标注和使用 `{}` 来围住函数体都是可选的。
    // 这些匿名函数（nameless function）被赋值给合适地命名的变量。
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;
    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    let mut count = 0;
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };
    inc();
    inc();

    // 不可复制类型（non-copy type）。
    let mut movable = Box::new(3);
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("movable: {:?}", movable);
        mem::drop(movable);
    };
    consume();

    // 使用 move 会强制闭包取得被捕获变量的所有权
    let v = vec![1, 2, 3];
    let contains = move |x| v.contains(x);
    println!("{}", contains(&1));
    println!("{}", contains(&11));
    // println!("{:?}", v);

    // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
    let double = |x| x * 4;
    println!("3 doubled: {}", apply_to_3(double));

    let mut add_100 = create_fn_mut(100);
    println!("{}", add_100(11));

    // std 中的例子
    let v2 = vec![1, 2, 3, 4];
    println!("{:?}", v2.iter().any(|&x| x == 2));
    println!("{:?}", v2.iter().find(|&&x| x == 2));

    // 高阶函数
    let upper = 1000;
    let sum_of_squared_odd_numbers: u32 = (0..).map(|x| x * x)
        .take_while(|&x| x < upper)
        .filter(|&x| is_odd(x))
        .fold(0, |x, i| x + i);
    println!("{}", sum_of_squared_odd_numbers);

    // 发散函数
    // 发散函数（diverging function）绝不会返回。 它们使用 ! 标记，这是一个空类型。
    let a = foo();
}

// Fn：表示捕获方式为通过引用（&T）的闭包
// FnMut：表示捕获方式为通过可变引用（&mut T）的闭包
// FnOnce：表示捕获方式为通过值（T）的闭包

// 输入闭包，返回一个 `i32` 整型的函数。
fn apply_to_3<F>(f: F) -> i32 where
// 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
    F: Fn(i32) -> i32 {
    f(3)
}

// 闭包作为输出参数
fn create_fn_mut(n: i32) -> impl FnMut(i32) -> i32 {
    move |x| x + n
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn foo() -> ! {
    panic!("This call never returns.");
}
