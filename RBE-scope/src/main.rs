use std::fmt::Debug;

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

fn main() {
    // 所有权和移动
    // 部分移动
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("xm"),
        age: 12,
    };
    // `name` 从 person 中移走，但 `age` 只是引用
    let Person { name, ref age } = person;
    println!("name is {}", name);
    println!("age is {}", age);
    // 报错！部分移动值的借用：`person` 部分借用产生
    // println!("The person struct is {:?}", person);
    // 但 `person.age` 因为没有被移动而可以继续使用
    println!("The person's age from person struct is {}", person.age);

    // ref 模式
    let c = 'Q';
    // 赋值语句中左边的 `ref` 关键字等价于右边的 `&` 符号
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 equals ref_c2: {}", ref_c1 == ref_c2);

    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 0, y: 0 };
    let _copy_of_x = {
        // `ref_to_x` 是一个指向 `point` 的 `x` 字段的引用。
        let Point { x: ref ref_to_x, y: _ } = point;
        // 返回一个 `point` 的 `x` 字段的拷贝。
        *ref_to_x
    };
    let mut mutable_point = point;
    {
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;
        *mut_ref_to_y = 1;
    }
    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // 包含一个指针的可变元组
    let mut mutable_tuple = (Box::new(5_u32), 3_u32);
    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2_u32;
    }
    println!("tuple is {:?}", mutable_tuple);

    // 生命周期
    // trait
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);

    // 约束
    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);

    fn print<T>(t: T) where
        T: Debug {
        println!("`print`: t is {:?}", t);
    }

    fn print_ref<'a, T>(t: &'a T) where
        T: Debug + 'a {
        println!("`print_ref`: t is {:?}", t);
    }

    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print(ref_x);

    // 强制转换
    // `<'a: 'b, 'b>` 读作生命周期 `'a` 至少和 `'b` 一样长。
    fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
        first
    }
    let first = 2;
    {
        let second = 3;
        println!("{} is the first", choose_first(&first, &second));
    };
}
