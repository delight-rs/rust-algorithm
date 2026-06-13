//! 两个数组的交集 II
//! https://leetcode.cn/problems/intersection-of-two-arrays-ii/description/?envType=problem-list-v2&envId=xulyhlrg
//! 给你两个整数数组 nums1 和 nums2 ，请你以数组形式返回两数组的交集。返回结果中每个元素出现的次数，应与元素在两个数组中都出现的次数一致
//! （如果出现次数不一致，则考虑取较小值）。可以不考虑输出结果的顺序。

use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums1.len());
        let count_map1 = get_counts_map(&nums1);
        let count_map2 = get_counts_map(&nums2);

        for n in count_map1.keys() {
            if count_map2.contains_key(n) {
                let count1 = count_map1.get(n).unwrap();
                let count2 = count_map2.get(n).unwrap();
                let count = if count1 < count2 { count1 } else { count2 };

                let n = **n;
                result.append(&mut vec![n; *count]);
            }
        }

        result
    }

    pub fn intersect_2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums1.len());
        let mut map = get_counts_map(&nums1);

        for n in nums2 {
            if let Some(count) = map.get_mut(&n) {
                if *count > 0 {
                    result.push(n);
                    *count -= 1
                }
            }
        }

        result
    }
}

fn get_counts_map(nums: &Vec<i32>) -> HashMap<&i32, usize> {
    let mut map = HashMap::new();

    for n in nums {
        map.entry(n).and_modify(|v| *v += 1).or_insert(1);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_of_two_arrays_ii() {
        assert_eq!(
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
        assert_eq!(
            Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![4, 9]
        );

        // 更多的test case
        assert_eq!(
            Solution::intersect(vec![1, 2, 3, 4], vec![3, 4, 5, 6]),
            vec![3, 4]
        );
        assert_eq!(
            Solution::intersect(vec![1, 2, 3, 4], vec![5, 6, 7, 8]),
            vec![]
        );
        assert_eq!(Solution::intersect(vec![1, 2, 2, 1], vec![2]), vec![2]);
    }
}
