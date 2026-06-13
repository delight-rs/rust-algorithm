//! 169. 多数元素
//! https://leetcode.cn/problems/majority-element/description/?envType=problem-list-v2&envId=xulyhlrg

use std::collections::HashMap;

struct Solution;

impl Solution {
    // 使用 hashmap 统计数量求解
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let count = nums.len() >> 1;

        for n in nums {
            map.entry(n).and_modify(|x| *x += 1).or_insert(1);
        }

        let result = map.iter().find(|(key, v)| v > &&count).unwrap();
        *result.0
    }

    // 使用摩尔投票法，参考 https://leetcode.cn/problems/majority-element/solutions/2362000/169-duo-shu-yuan-su-mo-er-tou-piao-qing-ledrh/?envType=problem-list-v2&envId=xulyhlrg
    pub fn majority_element_2(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut x = 0; // x 众数

        for n in nums {
            if sum == 0 {
                x = n;
            }

            let v = if n == x { 1 } else { -1 };
            sum = sum + v;
        }

        x
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_majority_element() {
        let nums = [2, 2, 1, 1, 1, 2, 2, 2].to_vec();
        let n = Solution::majority_element(nums);
        assert_eq!(n, 2);
    }

    #[test]
    fn test_majority_element_2() {
        let nums = [2, 2, 1, 1, 1, 2, 2, 2].to_vec();
        let n = Solution::majority_element_2(nums);
        assert_eq!(n, 2);
    }
}
