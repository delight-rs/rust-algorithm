//! 只出现一次的数字
//! https://leetcode.cn/problems/single-number/description/?envType=problem-list-v2&envId=xulyhlrg

use std::collections::HashSet;

struct Solution;

impl Solution {
    // 使用 hash
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();

        for n in nums {
            if set.contains(&n) {
                set.remove(&n);
            } else {
                set.insert(n);
            }
        }

        *set.iter().next().unwrap()
    }

    // 使用 异或
    pub fn single_number_2(mut nums: Vec<i32>) -> i32 {
        let result = nums
            .iter_mut()
            .reduce(|acc, n| {
                *acc ^= *n;
                return acc;
            })
            .unwrap();
        *result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_single_number() {
        let nums = [4, 1, 2, 1, 2].to_vec();
        let single_n = Solution::single_number(nums);
        assert_eq!(single_n, 4);
    }

    #[test]
    fn test_single_number_2() {
        let nums = [4, 1, 2, 1, 2].to_vec();
        let single_n = Solution::single_number_2(nums);
        assert_eq!(single_n, 4);
    }
}
