use std::fmt::{self, Formatter};
use std::mem;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "( {} {} )", self.0, self.1);
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn main() {
    // 元组
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);
    // 数组
    let v = [1; 3];
    println!("{:?}", v);
    // 数组是在栈中分配的
    println!("array occupies {} bytes", mem::size_of_val(&v));

    // 数组可以自动被借用为切片
    analyze_slice(&v);
    // 切片
    println!("{:?}", &v[0..2]);
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
