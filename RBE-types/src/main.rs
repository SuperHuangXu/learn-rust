fn main() {
    let decimal = 65.45_f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    let x = 1u8;
    let y: u32 = 2;
    let z = 3_f32;
    let i = 1;
    let f = 1.0;
    // `size_of_val` 返回一个变量所占的字节数
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // 别名，类型的名字必须遵循驼峰命名法
    type NanoSecond = u64;
    type Inch = u64;

    let nanoseconds: NanoSecond = 5;
    let inch = 5 as Inch;
    println!("{}, {}", nanoseconds, inch);
}
