use std::io;

fn hello_world() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("错误");
    println!("Hello, world! {}", s);
}
