// https://leetcode-cn.com/problems/remove-element/

fn main() {}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut ans = 0;
    let length = nums.len();
    for i in 0..length {
        if nums[i] != val {
            nums[ans] = nums[i];
            ans += 1;
        }
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use crate::remove_element;

    #[test]
    fn test1() {
        let mut v = vec![3, 2, 2, 3];
        assert_eq!(remove_element(&mut v, 3), 2);
    }

    #[test]
    fn test2() {
        let mut v = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(remove_element(&mut v, 2), 5);
    }
}
