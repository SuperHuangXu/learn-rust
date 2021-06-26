pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn panic_test() {
    panic!("panic_test error")
}

pub fn hello() -> String {
    "hello".to_string()
}


#[cfg(test)]
mod tests {
    use crate::{add_two, panic_test};

    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[should_panic(expected = "panic_test")]
    fn it_panic_test() {
        panic_test();
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four."))
        }
    }
}
