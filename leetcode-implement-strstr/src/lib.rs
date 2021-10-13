pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack_len = haystack.len();
    let needle_len = needle.len();

    if needle_len == 0 {
        return 0;
    }

    if haystack_len > 0 && needle_len > 0 {
        for i in 0..haystack_len {
            let current_len = i + needle_len;
            if current_len > haystack_len {
                break;
            }
            let s = &haystack[i..current_len];
            if needle.eq(s) {
                return i as i32;
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use crate::str_str;

    #[test]
    fn test1() {
        let res = str_str(String::from("hello"), String::from("ll"));
        assert_eq!(res, 2);
    }

    #[test]
    fn test2() {
        let res = str_str(String::from("aaaaa"), String::from("bba"));
        assert_eq!(res, -1);
    }

    #[test]
    fn test3() {
        let res = str_str(String::from(""), String::from(""));
        assert_eq!(res, 0);
    }

    #[test]
    fn test4() {
        let res = str_str(String::from(""), String::from("a"));
        assert_eq!(res, -1);
    }

    #[test]
    fn test5() {
        let res = str_str(String::from("a"), String::from(""));
        assert_eq!(res, 0);
    }

    #[test]
    fn test6() {
        let res = str_str(String::from("a"), String::from("a"));
        assert_eq!(res, 0);
    }
}



