// 名为 say_hello 的宏
macro_rules! say_hello {
    () => (
        println!("Hello!");
    );
}

macro_rules! create_function {
    ($func_name: ident) => (
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name))
        }
    );
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression: expr) => (
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression)
    );
}

// 这里列出全部指示符：
// block
// expr 用于表达式
// ident 用于变量名或函数名
// item
// pat (模式 pattern)
// path
// stmt (语句 statement)
// tt (标记树 token tree)
// ty (类型 type)

// 重载
macro_rules! test {
    ($left: expr; and $right: expr) => (
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    );
    ($left: expr; or $right: expr) => (
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    )
}

// 重复
// 宏在参数列表中可以使用 + 来表示一个参数可能出现一次或多次，使用 * 来表示该 参数可能出现零次或多次。
macro_rules! find_min {
    // 基本情况
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), +) => (
        std::cmp::min($x, find_min!($($y), +))
    )
}

fn main() {
    println!("Hello, world!");
    say_hello!();

    foo();
    bar();

    print_result!({
        let x = 1_u32;
        x * x + 2 * x - 1
    });

    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);

    println!("{}", find_min!(1));
    println!("{}", find_min!(1+2, 3, 4*2,0));
}
