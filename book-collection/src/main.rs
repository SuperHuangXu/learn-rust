use std::collections::HashMap;

fn main() {}

fn hashmap_demo() {
    let mut hm = HashMap::new();
    hm.insert(String::from("Blue"), 10);
    hm.insert(String::from("Yellow"), 20);
    // 遍历 hm
    for i in hm.iter() {
        println!("{}: {}", i.0, i.1)
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 20];
    // 通过 zip 方法聚合两个 vec
    let scores: HashMap<_, _> = teams.iter().zip(intial_scores).collect();
    for i in scores.iter() {
        println!("{}: {}", i.0, i.1)
    }

    // 通过 key 获取 value
    if let Some(val) = scores.get(&String::from("Yellow")) {
        println!("Yellow is {}", val);
    }
    match scores.get(&String::from("Blue")) {
        None => {}
        Some(val) => {
            println!("Blue is {}", val);
        }
    }

    // 在 key 不对应任何值的情况下，才插入 value
    hm.entry(String::from("Red")).or_insert(30);
    println!("{:?}", &hm);

    // 基于现有 value 改变 value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for i in text.split_whitespace() {
        let cont = map.entry(i).or_insert(0);
        *cont += 1;
    }
    println!("{:#?}", &map);
}

fn str_demo() -> String {
    let mut s = String::from("hello");
    s.push(' ');
    s.push_str("world~");

    let s1 = String::from("mo ");
    let s2 = s1 + &s;


    let s1 = String::from("mo ");
    let s3 = format!("{}-{}", s1, s2);

    s3
}

fn vec_loop() -> Vec<i32> {
    let mut v2 = vec![100, 200, 300];
    for i in &mut v2 {
        *i += 50;
    }
    v2
}

fn vec_get() -> Option<i32> {
    let v = vec![1, 2, 3, 4, 5];
    match v.get(2) {
        Option::Some(third) => Some(*third),
        Option::None => None
    }
}

#[cfg(test)]
mod tests {
    use crate::{vec_get, vec_loop, str_demo};

    #[test]
    fn it_vec_get() {
        let res = vec_get();
        assert_eq!(res.unwrap(), 3);
    }

    #[test]
    fn it_vec_loop() {
        let res = vec_loop();
        assert_eq!(res, [150, 250, 350]);
    }

    #[test]
    fn it_str_demo() {
        let res = str_demo();
        assert_eq!(res, "mo -mo hello world~");
    }
}
