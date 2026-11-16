//! 丢失的数字
//! https://leetcode.cn/problems/missing-number/description/?envType=problem-list-v2&envId=xulyhlrg

use std::collections::HashSet;

struct Solution;

impl Solution {
    // 使用 hashset
    // 1. 将 数组中 的数字放入HashSet 中
    // 2. 遍历 [0..n] 的数，如果在 HastSet 中没有找到，返回该数
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        let mut set = HashSet::with_capacity(length);

        for n in nums {
            set.insert(n);
        }

        for i in 0..=length {
            let n = i as i32;
            if !set.contains(&n) {
                return n;
            }
        }

        -1
    }

    // 使用 数学 方法
    // [1,n] 的 和减去 nums的和即为结果
    pub fn missing_number_2(mut nums: Vec<i32>) -> i32 {
        let sum1 = nums.iter_mut().fold(0, |mut acc, n| {
            acc += *n;
            return acc;
        });

        let sum2: i32 = (0..=nums.len()).fold(0, |mut acc, n| {
            acc = acc + n as i32;
            return acc;
        });

        sum2 - sum1
    }

    // 使用异或位运算
    // 1. 对于异或位运输 x ^=x -> 0, x ^= 0 -> x，且异或满足交换律
    // 2. 因此可以对数组追加 [0,n]的值，然后遍历数组进行异或运输，最后的值就是丢失的数值
    pub fn missing_number_3(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let lenght = nums.len();

        for n in nums {
            result ^= n;
        }

        for n in 0..=lenght {
            result ^= n as i32;
        }

        result
    }
}

#[cfg(test)]
mod test {

    use super::Solution;

    #[test]
    fn test_missing_number() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }

    #[test]
    fn test_missing_number_2() {
        assert_eq!(Solution::missing_number_2(vec![3, 0, 1]), 2);
        assert_eq!(Solution::missing_number_2(vec![0, 1]), 2);
        assert_eq!(
            Solution::missing_number_2(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
            8
        );
    }

    #[test]
    fn test_missing_number_3() {
        assert_eq!(Solution::missing_number_3(vec![3, 0, 1]), 2);
        assert_eq!(Solution::missing_number_3(vec![0, 1]), 2);
        assert_eq!(
            Solution::missing_number_3(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
            8
        );
    }
}
