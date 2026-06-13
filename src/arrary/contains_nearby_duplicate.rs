//! 存在重复元素 II
//! https://leetcode.cn/problems/contains-duplicate-ii/description/?envType=problem-list-v2&envId=xulyhlrg

use std::collections::{HashMap, hash_map::Entry};

struct Solution;

impl Solution {
    // 遍历，满足条件即可, 但会超出时间限制
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let length = nums.len();

        if length <= 1 {
            return false;
        }

        let k = if k as usize >= length {
            length - 1
        } else {
            k as usize
        };

        for step in 1..=k {
            for i in 0..length - step {
                if nums[i] == nums[i + step] {
                    return true;
                }
            }
        }

        false
    }

    // 1. 使用 map 记录元素的索引位置
    // 2. 求出每个元素最小相邻间距 step
    // 3. 判断 step 是否小于 k
    pub fn contains_nearby_duplicate_2(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() <= 1 {
            return false;
        }

        let mut index_map: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            index_map
                .entry(*n)
                .and_modify(|v| {
                    v.push(i);
                })
                .or_insert(vec![i]);
        }

        let mut flag = false;
        index_map.iter().for_each(|(key, value)| {
            let mut min_step = usize::MAX;
            for i in 0..value.len() - 1 {
                let step = value[i + 1] - value[i];
                if step < min_step {
                    min_step = step;
                }
            }

            if min_step <= k as usize {
                flag = true;
                return;
            }
        });

        flag
    }

    // 1. 使用 map 记录元素的记录元素的一个索引位置 min_i
    // 2. 当前元素 nums[i] 是否存在 map中 且, i-min_i 是否 <=k
    // 3. 满足条件 2 则返回 true, 不满足则 map 中插入 (nums[i], i)
    pub fn contains_nearby_duplicate_3(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<&i32, usize> = HashMap::with_capacity(nums.len());
        let k = k as usize;

        for (i, n) in nums.iter().enumerate() {
            if map.contains_key(n) && i - map.get(n).unwrap() <= k {
                return true;
            }
            map.insert(n, i);
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_contains_nearby_duplicate() {
        let nums = [1, 2, 3, 1].to_vec();
        let nums_2 = [1, 0, 1, 1].to_vec();
        let nums_3 = [1, 2, 3, 1, 2, 3].to_vec();
        let nums_4 = [99, 99].to_vec();

        assert!(Solution::contains_nearby_duplicate(nums, 3));
        assert!(Solution::contains_nearby_duplicate(nums_2, 1));
        assert_eq!(Solution::contains_nearby_duplicate(nums_3, 2), false);
        assert!(Solution::contains_nearby_duplicate(nums_4, 2));
    }

    #[test]
    fn test_contains_nearby_duplicate_2() {
        let nums = [1, 2, 3, 1].to_vec();
        let nums_2 = [1, 0, 1, 1].to_vec();
        let nums_3 = [1, 2, 3, 1, 2, 3].to_vec();
        let nums_4 = [99, 99].to_vec();

        assert!(Solution::contains_nearby_duplicate_2(nums, 3));
        assert!(Solution::contains_nearby_duplicate_2(nums_2, 1));
        assert_eq!(Solution::contains_nearby_duplicate_2(nums_3, 2), false);
        assert!(Solution::contains_nearby_duplicate_2(nums_4, 2));
    }

    #[test]
    fn test_contains_nearby_duplicate_3() {
        let nums = [1, 2, 3, 1].to_vec();
        let nums_2 = [1, 0, 1, 1].to_vec();
        let nums_3 = [1, 2, 3, 1, 2, 3].to_vec();
        let nums_4 = [99, 99].to_vec();

        assert!(Solution::contains_nearby_duplicate_3(nums, 3));
        assert!(Solution::contains_nearby_duplicate_3(nums_2, 1));
        assert_eq!(Solution::contains_nearby_duplicate_3(nums_3, 2), false);
        assert!(Solution::contains_nearby_duplicate_3(nums_4, 2));
    }
}
