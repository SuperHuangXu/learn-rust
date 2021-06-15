use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("随机数生成~");
    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("猜一下数字");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("error");

        print!("猜的数字：{}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("<"),
            Ordering::Greater => println!(">"),
            Ordering::Equal => {
                println!("=");
                break;
            }
        }
    }
}
