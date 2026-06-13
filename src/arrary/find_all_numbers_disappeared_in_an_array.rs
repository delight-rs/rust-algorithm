use std::{collections::HashSet, num};

struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len();
        let mut set: HashSet<i32> = nums.into_iter().collect();
        let mut result = vec![];

        for i in 1..=length {
            let n = i as i32;
            if !set.contains(&n) {
                result.push(n);
            }
        }

        result
    }

    // 使用 vec 作为额外空间
    pub fn find_disappeared_numbers_2(nums: Vec<i32>) -> Vec<i32> {
        let mut vec = vec![0; nums.len() + 1];

        for n in nums {
            vec[n as usize] = 1;
        }

        let mut result = vec![];
        for i in 1..vec.len() {
            if vec[i] == 0 {
                result.push(i as i32);
            }
        }

        result
    }

    // 使用 原数组 作为额外空间
    pub fn find_disappeared_numbers_3(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let n = nums[i];

            if n != 0 {
                let mut j = (n - 1) as usize;

                while nums[j] != 0 {
                    let next = (nums[j] - 1) as usize;
                    nums[j] = 0;
                    j = next;
                }
            }
        }

        let mut result = vec![];

        for i in 0..nums.len() {
            if nums[i] != 0 {
                result.push((i + 1) as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_disappeared_numbers() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );

        assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
        assert_eq!(Solution::find_disappeared_numbers(vec![2, 2]), vec![1]);
        assert_eq!(Solution::find_disappeared_numbers(vec![1, 2]), vec![]);
    }

    #[test]
    fn test_find_disappeared_numbers_2() {
        assert_eq!(
            Solution::find_disappeared_numbers_2(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );

        assert_eq!(Solution::find_disappeared_numbers_2(vec![1, 1]), vec![2]);
        assert_eq!(Solution::find_disappeared_numbers_2(vec![2, 2]), vec![1]);
        assert_eq!(Solution::find_disappeared_numbers_2(vec![1, 2]), vec![]);
    }

    #[test]
    fn test_find_disappeared_numbers_3() {
        assert_eq!(
            Solution::find_disappeared_numbers_3(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );

        assert_eq!(Solution::find_disappeared_numbers_3(vec![1, 1]), vec![2]);
        assert_eq!(Solution::find_disappeared_numbers_3(vec![2, 2]), vec![1]);
        assert_eq!(Solution::find_disappeared_numbers_3(vec![1, 2]), vec![]);
    }

    #[test]
    fn test_find_disappeared_numbers_4() {}
}
