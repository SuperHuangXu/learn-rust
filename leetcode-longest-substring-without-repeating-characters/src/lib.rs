// https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut str: Vec<char> = vec![];
    let mut max_len = 0;
    for value in s.chars() {
        let find_index = str.iter().position(|x| x == &value);
        match find_index {
            None => {
                str.push(value);
                if max_len < str.len() {
                    max_len = str.len();
                }
            }
            Some(_) => {
                str = str[find_index.unwrap() + 1..].to_vec();
                str.push(value);
            }
        }
    }
    max_len as i32
}

#[cfg(test)]
mod tests {
    use crate::length_of_longest_substring;

    #[test]
    fn it_works1() {
        let s = String::from("abcabcbb");
        let output = 3;
        assert_eq!(length_of_longest_substring(s), output);
    }

    #[test]
    fn it_works2() {
        let s = String::from("bbbbb");
        let output = 1;
        assert_eq!(length_of_longest_substring(s), output);
    }

    #[test]
    fn it_works3() {
        let s = String::from("pwwkew");
        let output = 3;
        assert_eq!(length_of_longest_substring(s), output);
    }

    #[test]
    fn it_works4() {
        let s = String::from("");
        let output = 0;
        assert_eq!(length_of_longest_substring(s), output);
    }

    #[test]
    fn it_works5() {
        let s = String::from(" ");
        let output = 1;
        assert_eq!(length_of_longest_substring(s), output);
    }

    #[test]
    fn it_works6() {
        let s = String::from("aabaab!bb");
        let output = 3;
        assert_eq!(length_of_longest_substring(s), output);
    }
}
