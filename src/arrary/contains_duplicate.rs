//! 存在重复元素
//! https://leetcode.cn/problems/contains-duplicate/description/?envType=problem-list-v2&envId=xulyhlrg

use std::collections::HashSet;

struct Solution;

impl Solution {
    // 1. 使用 hashset 求解
    // 2. 创建一个 set
    // 3. 将元素存入 set前先判断 set 中是否有该元素，有则返回 true, 否则将元素插入 set
    // 4. 循环结束，没有提前跳出循环则返回 false
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::with_capacity(nums.len()); //提前分配好 Set大小可以减少执行时间

        for n in nums {
            if !set.insert(n) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_contains_duplicate() {
        let nums = [1, 2, 3, 1].to_vec();
        assert!(Solution::contains_duplicate(nums));
    }
}
