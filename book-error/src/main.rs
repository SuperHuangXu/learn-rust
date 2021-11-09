use std::fs::File;
use std::io::{self, ErrorKind, Error, Read};

fn main() {
    let res = read_file("Cargo.toml");
    println!("{}", res.unwrap());
}

fn read_file(file_name: &str) -> Result<String, io::Error> {
    // let f = File::open(file_name);
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e)
    // };
    //
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e)
    // }

    // 使用 ? 运算符
    // let mut f = File::open(file_name)?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // 使用链式调用
    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
}

fn open_file() {
    let file_name = "hello.txt";

    let f = File::open(file_name);
    let f = match f {
        Ok(file) => {
            file
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),
            }
            other_error => panic!("Error opening file: {:?}", other_error)
        }
    };

    // unwrap 是 match 的语法糖，若 ok 则返回 ok 内数据，否则执行 err
    let f = File::open(file_name).unwrap();
    // expect 和 unwrap 的区别是可以自定义错误信息
    let f = File::open(file_name).expect("自定义error message");

    // 使用闭包重写
    let f = File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error)
            })
        } else {
            panic!("Error opening file: {:?}", error)
        }
    });
}
