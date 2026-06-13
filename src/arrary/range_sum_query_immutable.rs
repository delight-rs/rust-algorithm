//! 区域和检索 - 数组不可变
//! https://leetcode.cn/problems/range-sum-query-immutable/description/?envType=problem-list-v2&envId=xulyhlrg
//! 无符号数(usize) 表示数字只能取正数和 0

struct NumArray {
    inner: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self { inner: nums }
    }

    // 方法 1 暴力遍历
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;

        let sum = self.inner[left..right + 1].iter().fold(0, |mut acc, n| {
            acc += *n;
            acc
        });

        sum
    }

    // 方法2 创建时记录每个 index 的 0..=index 的和
    // 然后再通过 sums[right] - sums[left-1] 直接获取区间和
    fn new_2(nums: Vec<i32>) -> Self {
        let length = nums.len();

        if length == 0 {
            return Self { inner: nums };
        }

        let mut sums = Vec::with_capacity(length);
        sums.push(nums[0]);

        for i in 1..length {
            sums.push(nums[i] + sums[i - 1]);
        }

        Self { inner: sums }
    }

    fn sum_range_2(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;

        if left == 0 {
            self.inner[right]
        } else {
            self.inner[right] - self.inner[left - 1]
        }
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */

mod test {
    use super::*;

    #[test]
    fn test_range_sum_query_immutable() {
        let nums = vec![-2, 0, 3, -5, 2, -1];
        let obj = NumArray::new(nums);
        assert_eq!(obj.sum_range(0, 2), 1);
        assert_eq!(obj.sum_range(2, 5), -1);
        assert_eq!(obj.sum_range(0, 5), -3);
    }

    #[test]
    fn test_range_sum_query_immutable_2() {
        let nums = vec![-2, 0, 3, -5, 2, -1];
        let obj = NumArray::new_2(nums);

        assert_eq!(obj.sum_range_2(0, 2), 1);
        assert_eq!(obj.sum_range_2(2, 5), -1);
        assert_eq!(obj.sum_range_2(0, 5), -3);
    }
}
