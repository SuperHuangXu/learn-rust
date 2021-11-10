use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::Add;

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

// 不可复制的类型
struct Empty;

struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// 对泛型的调用者类型 `U` 和任何泛型类型 `T` 实现 `DoubleDrop<T>` 。
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.length
    }
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn get_area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T where
    Option<T>: Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let x = Val { val: 111_f64 };
    let y = GenVal { gen_val: 100_u32 };
    println!("x: {}, y: {}", x.value(), y.value());

    // trait
    let e = Empty;
    let n = Null;
    e.double_drop(n);

    // 约束
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    print_debug(&rectangle);
    println!("Area: {}", get_area(&rectangle));
    let _triangle = Triangle { length: 4.0, height: 5.0 };
    // print_debug(&_triangle);
    // println!("Area: {}", get_area(&_triangle));

    // 多重约束
    let s = "words";
    let arr = [1, 2, 3];
    let vec = vec![1, 2, 3];
    compare_prints(&s);
    // arr 没有实现 Display trait
    // compare_prints(&arr);
    compare_types(&arr, &vec);

    // where 子句
    let vec = vec![1, 2, 3];
    vec.print_in_option();

    // new type - 即为不同种类的数据分别定义新的类型
    struct Years(i64);
    struct Days(i64);

    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }

    impl Days {
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }

    fn old_enough(age: &Years) -> bool {
        age.0 >= 18
    }

    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("{}", old_enough(&age_days));

    // 关联项
    struct Container(i32, i32);

    // 这个 trait 检查给定的 2 个项是否储存于容器中，并且能够获得容器的第一个或最后一个值。
    trait Contains {
        // 在这里定义可以被方法使用的泛型类型。
        type A;
        type B;

        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container {
        type A = i32;
        type B = i32;

        fn contains(&self, n1: &i32, n2: &i32) -> bool {
            (&self.0 == n1) && (&self.1 == n2)
        }
        fn first(&self) -> i32 {
            self.0
        }
        fn last(&self) -> i32 {
            self.1
        }
    }

    fn difference<C>(c: &C) -> i32 where
        C: Contains {
        c.last() - c.first()
    }

    let n1 = 3;
    let n2 = 10;
    let container = Container(n1, n2);
    println!("Does container contain {} and {}: {}", &n1, &n2, container.contains(&n1, &n2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is: {}", difference(&container));

    // 虚类型参数
    // 虚类型（phantom type）参数是一种在运行时不出现，而在（且仅在）编译时进行静态检查 的类型参数。
    #[derive(Debug, Clone, Copy)]
    enum Inch {}
    #[derive(Debug, Clone, Copy)]
    enum Mm {}

    #[derive(Debug, Clone, Copy)]
    struct Length<Unit>(f64, PhantomData<Unit>);

    impl<Unit> Add for Length<Unit> {
        type Output = Length<Unit>;
        fn add(self, rhs: Length<Unit>) -> Length<Unit> {
            Length(self.0 + rhs.0, PhantomData)
        }
    }

    let one_foot = Length::<Inch>(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;
    println!("{:?}", two_feet);
    println!("{:?}", two_meters);
    // Error: expected `Length<Inch>`, found `Length<Mm>`
    // let one_fetter = one_foot + one_meter;
}
