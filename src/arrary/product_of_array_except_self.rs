// https://leetcode.com/problems/product-of-array-except-self/?envType=study-plan-v2&envId=leetcode-75

struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n];

        // 第一遍求 prefix = nums[0..i]
        let mut prefix = 1;
        for i in 0..n {
            result[i] = prefix;
            prefix *= nums[i];
        }

        // 第二遍求及最终结果 suffix = nums[i+1..n].rev()
        let mut suffix = 1;
        for i in (0..n).rev() {
            result[i] *= suffix;
            suffix *= nums[i];
        }

        result
    }

    pub fn product_except_self_2(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            let mut acc = 1;

            for j in 0..nums.len() {
                if j != i {
                    acc *= nums[j];
                }
            }

            result.push(acc);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_product_except_self_example1() {
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(Solution::product_except_self(nums), vec![24, 12, 8, 6]);
    }

    #[test]
    fn test_product_except_self_example2_with_zero() {
        let nums = [-1, 1, 0, -3, 3].to_vec();
        assert_eq!(Solution::product_except_self(nums), vec![0, 0, 9, 0, 0]);
    }

    #[test]
    fn test_product_except_self_minimum_length() {
        let nums = [3, 5].to_vec();
        assert_eq!(Solution::product_except_self(nums), vec![5, 3]);
    }

    #[test]
    fn test_product_except_self_multiple_zeros() {
        let nums = [0, 0, 4, 5].to_vec();
        assert_eq!(Solution::product_except_self(nums), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_product_except_self_all_negative() {
        let nums = [-1, -2, -3].to_vec();
        assert_eq!(Solution::product_except_self(nums), vec![6, 3, 2]);
    }

    #[test]
    fn test_product_except_self_length_two_with_zero() {
        let nums = [0, -5].to_vec();
        assert_eq!(Solution::product_except_self(nums), vec![-5, 0]);
    }

    #[test]
    fn test_product_except_self_mixed_signs() {
        let nums = [1, -1, 1, -1].to_vec();
        assert_eq!(Solution::product_except_self(nums), vec![1, -1, 1, -1]);
    }

    #[test]
    fn test_product_except_self_contains_one() {
        let nums = [1, 2, 3].to_vec();
        assert_eq!(Solution::product_except_self(nums), vec![6, 3, 2]);
    }
}
