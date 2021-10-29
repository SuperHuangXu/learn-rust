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
    println!("array occupies {} bytes", mem::size_of_val(&v));
    println!("{:?}", &v[0..2]);
}
