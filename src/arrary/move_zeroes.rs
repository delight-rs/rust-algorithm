//! 移动零
//! https://leetcode.cn/problems/move-zeroes/description/?envType=problem-list-v2&envId=xulyhlrg

struct Solution;

impl Solution {
    // 采用双指针，将左指针第一 0， 与右指针非 0 交换
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;
        let mut right = 0;
        let mut length = nums.len();

        while right < length {
            let a = nums[left];
            let b = nums[right];

            if b != 0 {
                if a == 0 {
                    nums.swap(left, right);
                }
                left += 1;
            }

            right += 1;
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
}
