//! 移动零
//! CN https://leetcode.cn/problems/move-zeroes/description/?envType=problem-list-v2&envId=xulyhlrg
//! EN https://leetcode.com/problems/move-zeroes/description/?envType=study-plan-v2&envId=leetcode-75

struct Solution;

impl Solution {
    // 采用双指针，将左指针第一 0， 与右指针非 0 交换
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left: usize = 0;

        for next in 0..nums.len() {
            if nums[next] != 0 {
                if left != next {
                    nums.swap(left, next);
                }

                left += 1
            }
        }
    }

    // 先将所有非 0 的值填充到数组前部分，记录住 index
    // 然后 将数组后部分全部更改为 0
    pub fn move_zeroes_2(nums: &mut Vec<i32>) {
        let length = nums.len();
        let mut left = 0;

        for i in 0..length {
            let n = nums[i];
            if n != 0 {
                nums[left] = n;
                left += 1;
            }
        }

        for i in left..length {
            nums[i] = 0;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![1, 2, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 12, 0]);
    }

    #[test]
    fn test_move_zeroes_2() {
        let mut nums = vec![1, 2, 0, 3, 12];
        Solution::move_zeroes_2(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 12, 0]);
    }

    #[test]
    fn test_move_zeroes_example1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_move_zeroes_example2() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test_move_zeroes_all_zeros() {
        let mut nums = vec![0, 0, 0, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_move_zeroes_no_zeros() {
        let mut nums = vec![5, -2, 7, 7];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![5, -2, 7, 7]);
    }

    #[test]
    fn test_move_zeroes_negative_duplicates_extremes() {
        let mut nums = vec![i32::MIN, 0, -1, 0, -1, i32::MAX, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![i32::MIN, -1, -1, i32::MAX, 0, 0, 0]);
    }

    #[test]
    fn test_move_zeroes_max_length() {
        let mut nums = vec![0; 5_000];
        nums.extend(1..=5_000i32);

        let mut expected: Vec<i32> = (1..=5_000i32).collect();
        expected.resize(10_000, 0);

        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_move_zeroes_2_example1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes_2(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_move_zeroes_2_example2() {
        let mut nums = vec![0];
        Solution::move_zeroes_2(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test_move_zeroes_2_all_zeros() {
        let mut nums = vec![0, 0, 0, 0, 0];
        Solution::move_zeroes_2(&mut nums);
        assert_eq!(nums, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_move_zeroes_2_no_zeros() {
        let mut nums = vec![5, -2, 7, 7];
        Solution::move_zeroes_2(&mut nums);
        assert_eq!(nums, vec![5, -2, 7, 7]);
    }

    #[test]
    fn test_move_zeroes_2_negative_duplicates_extremes() {
        let mut nums = vec![i32::MIN, 0, -1, 0, -1, i32::MAX, 0];
        Solution::move_zeroes_2(&mut nums);
        assert_eq!(nums, vec![i32::MIN, -1, -1, i32::MAX, 0, 0, 0]);
    }

    #[test]
    fn test_move_zeroes_2_max_length() {
        let mut nums = vec![0; 5_000];
        nums.extend(1..=5_000i32);

        let mut expected: Vec<i32> = (1..=5_000i32).collect();
        expected.resize(10_000, 0);

        Solution::move_zeroes_2(&mut nums);
        assert_eq!(nums, expected);
    }
}
