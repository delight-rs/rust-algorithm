// CN: https://leetcode.cn/problems/increasing-triplet-subsequence/description/
// EN: https://leetcode.com/problems/increasing-triplet-subsequence/description/?envType=study-plan-v2&envId=leetcode-75

struct Solution {}

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = i32::MAX;
        let mut second = i32::MAX;

        for n in nums {
            if n > second {
                return true;
            } else if n > first {
                second = n;
            } else {
                first = n;
            }
        }

        false
    }

    // Brute Force Solution
    pub fn increasing_triplet_2(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] < nums[j] {
                    for k in j + 1..nums.len() {
                        if nums[j] < nums[k] {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_increasing_triplet_example1() {
        assert!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_increasing_triplet_example2() {
        assert!(!Solution::increasing_triplet(vec![5, 4, 3, 2, 1]));
    }

    #[test]
    fn test_increasing_triplet_example3() {
        assert!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]));
    }

    #[test]
    fn test_increasing_triplet_single_element() {
        assert!(!Solution::increasing_triplet(vec![1]));
    }

    #[test]
    fn test_increasing_triplet_two_elements() {
        assert!(!Solution::increasing_triplet(vec![1, 2]));
    }

    #[test]
    fn test_increasing_triplet_all_equal() {
        assert!(!Solution::increasing_triplet(vec![7, 7, 7, 7]));
    }

    #[test]
    fn test_increasing_triplet_duplicates_break_strict_order() {
        assert!(!Solution::increasing_triplet(vec![2, 2, 3]));
    }

    #[test]
    fn test_increasing_triplet_negative_and_zero() {
        assert!(Solution::increasing_triplet(vec![-3, 0, 1]));
    }

    #[test]
    fn test_increasing_triplet_all_negative_descending() {
        assert!(!Solution::increasing_triplet(vec![-1, -2, -3]));
    }

    #[test]
    fn test_increasing_triplet_i32_bounds() {
        assert!(Solution::increasing_triplet(vec![i32::MIN, 0, i32::MAX]));
    }

    #[test]
    fn test_increasing_triplet_non_consecutive_indices() {
        assert!(Solution::increasing_triplet(vec![5, 6, 3, 1, 0, 2, 3]));
    }

    #[test]
    fn test_increasing_triplet_late_small_values() {
        assert!(Solution::increasing_triplet(vec![20, 100, 10, 12, 5, 13]));
    }

    #[test]
    fn test_increasing_triplet_max_length_descending() {
        let nums: Vec<i32> = (0..500_000).rev().collect();
        assert!(!Solution::increasing_triplet(nums));
    }
}
