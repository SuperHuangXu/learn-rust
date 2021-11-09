use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    // closures();

    let v1 = vec![1, 2, 3];
    for i in v1 {
        println!("{}", i);
    }

    // 因为迭代器是惰性的，若不使用消耗型适配器方法，则下面的语句不会执行
    // v1.iter().map(|x| x + 1);
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:#?}", v2)
}


fn closures() {
    let intensity1 = 20;
    let intensity2 = 30;

    let mut expensive_result = Cache::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    println!(
        "Today, do {} pushups!",
        expensive_result.value(intensity1)
    );

    println!(
        "Today, do {} pushups!",
        expensive_result.value(intensity2)
    );
}

struct Cache<T>
    where T: Fn(u32) -> u32 {
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cache<T>
    where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        let map_value = self.value.get(&arg);
        match map_value {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}
