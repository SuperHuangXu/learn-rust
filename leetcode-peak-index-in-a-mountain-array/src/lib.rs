// https://leetcode-cn.com/problems/peak-index-in-a-mountain-array/

pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let mut maxIndex = 0;
    for (index, value) in arr.iter().enumerate() {
        if value > arr.get(maxIndex).unwrap() {
            maxIndex = index;
        }
    }
    maxIndex as i32
}

#[cfg(test)]
mod tests {
    use crate::peak_index_in_mountain_array;

    #[test]
    fn it_works1() {
        let v = vec![0, 1, 0];
        assert_eq!(peak_index_in_mountain_array(v), 1);
    }

    #[test]
    fn it_works2() {
        let v = vec![0, 2, 1, 0];
        assert_eq!(peak_index_in_mountain_array(v), 1);
    }

    #[test]
    fn it_works3() {
        let v = vec![0, 10, 5, 2];
        assert_eq!(peak_index_in_mountain_array(v), 1);
    }

    #[test]
    fn it_works4() {
        let v = vec![3, 4, 5, 1];
        assert_eq!(peak_index_in_mountain_array(v), 2);
    }
}
