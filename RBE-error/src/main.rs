#![allow(dead_code)]

use std::num::ParseIntError;

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

// 组合算子 map
#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
struct Peeled(Food);

#[derive(Debug)]
struct Chopped(Food);

#[derive(Debug)]
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        None => None,
        Some(Peeled(food)) => Some(Chopped(food)),
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        None => println!("Oh no! It wasn't edible."),
        Some(food) => println!("Mmm. I love {:?}", food),
    }
}

// Result
fn multiply(s1: &str, s2: &str) -> i32 {
    let n1 = s1.parse::<i32>().unwrap();
    let n2: i32 = s2.parse().unwrap();
    n1 * n2
}

// ?
fn multiply_2(s1: &str, s2: &str) -> Result<i32, ParseIntError> {
    let n1: i32 = s1.parse()?;
    let n2: i32 = s2.parse()?;
    Ok(n1 * n2)
}

// 处理多种错误类型
fn double_first(vec: &Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|x| x * 2)
    })
}

fn double_first_map(vec: &Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|x| x * 2)
    });
    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let code = Some(11);
    let person = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: code,
                number: 123456789,
            })
        })
    };

    assert_eq!(code, person.work_phone_area_code());

    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));

    let cookie_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cookie_potato);

    let twenty = multiply("10", "2");
    // multiply("t", "2");
    let _m2 = multiply_2("t", "2");
    println!("{}", twenty);

    let numbers = vec!["1", "2", "3"];
    let err_numbers = vec!["1to", "2", "3"];
    println!("{:?}", double_first(&numbers));
    println!("{:?}", double_first(&err_numbers));
    println!("{:?}", double_first_map(&numbers));
    println!("{:?}", double_first_map(&err_numbers));
}
