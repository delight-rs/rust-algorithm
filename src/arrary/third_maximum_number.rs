use std::collections::{BTreeSet, HashSet};

struct Solution;

impl Solution {
    // 1. 去重，排序
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::with_capacity(nums.len());
        let mut uniq_nums: Vec<i32> = nums.into_iter().filter(|n| seen.insert(*n)).collect();

        uniq_nums.sort_by(|a, b| b.cmp(a));

        if uniq_nums.len() <= 2 {
            uniq_nums[0]
        } else {
            uniq_nums[2]
        }
    }

    // 2. 使用 三个 变量 a, b, c来记录第1，2，3 个最大的数来解决这个问
    pub fn third_max_2(nums: Vec<i32>) -> i32 {
        let mut max1 = i32::MIN;
        let mut max2 = i32::MIN;
        let mut max3 = i32::MIN;

        let mut seen = HashSet::with_capacity(nums.len());
        let mut uniq_nums: Vec<i32> = nums.into_iter().filter(|n| seen.insert(*n)).collect();

        dbg!(max1);

        for n in uniq_nums {
            if n > max1 {
                max3 = max2; // 此时 max3 应为 max2
                max2 = max1; // 此时 max2 应为 max1
                max1 = n;
            } else if n > max2 && n < max1 {
                max3 = max2; // 此时 max3 应为 max2
                max2 = n;
            } else if n > max3 && n < max2 {
                max3 = n;
            }
        }

        if seen.len() >= 3 { max3 } else { max1 }
    }

    // 使用有序集合
    // 1. 创建一个 有序集合 BTreeSet
    // 2. 遍历数组，插入 n 如果 BTreeSet.length > 3  则移除 BTreeSet.min
    // 3. 遍历完数组，如果 BTreeSet.length >=3 则返回第一个元素，否则返回第三个
    pub fn third_max_3(nums: Vec<i32>) -> i32 {
        let mut tree_set = BTreeSet::new();

        for n in nums {
            tree_set.insert(n);

            if tree_set.len() > 3 {
                tree_set.pop_first();
            }
        }

        if tree_set.len() == 3 {
            *tree_set.first().unwrap()
        } else {
            *tree_set.last().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_third_max() {
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
        assert_eq!(Solution::third_max(vec![1, 2]), 2);
        assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    }

    #[test]
    fn test_third_max_2() {
        assert_eq!(Solution::third_max_2(vec![1, 2, 2, 5, 3, 5]), 2);
        assert_eq!(Solution::third_max_2(vec![2, 2, 2, 1]), 2);
        assert_eq!(Solution::third_max_2(vec![2, 2, 3, 1]), 1);
        assert_eq!(Solution::third_max_2(vec![1, 2, -2147483648]), -2147483648);
    }

    #[test]
    fn test_third_max_3() {
        assert_eq!(Solution::third_max_3(vec![1, 2, 2, 5, 3, 5]), 2);
        assert_eq!(Solution::third_max_3(vec![2, 2, 2, 1]), 2);
        assert_eq!(Solution::third_max_3(vec![2, 2, 3, 1]), 1);
        assert_eq!(Solution::third_max_3(vec![1, 2, -2147483648]), -2147483648);
    }
}
