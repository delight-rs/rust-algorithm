// https://leetcode.com/problems/can-place-flowers/description/?envType=study-plan-v2&envId=leetcode-75

struct Solution {}

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut remaining = n;

        for i in 0..flowerbed.len() {
            let left_empty = i == 0 || flowerbed[i - 1] == 0;
            let right_empty = i + 1 == flowerbed.len() || flowerbed[i + 1] == 0;

            if flowerbed[i] == 0 && left_empty && right_empty {
                remaining -= 1;
                flowerbed[i] = 1;
            }

            if remaining == 0 {
                return true;
            }
        }

        remaining <= 0
    }

    pub fn can_place_flowers_2(mut flowerbed: Vec<i32>, n: i32) -> bool {
        if flowerbed.len() == 1 {
            let max_count = if flowerbed[0] == 0 { 1 } else { 0 };
            return max_count >= n;
        }

        let mut max_count = 0;

        for i in 0..flowerbed.len() {
            if max_count >= n {
                return true;
            }

            if flowerbed[i] == 0 {
                if i == 0 {
                    if flowerbed[1] == 0 {
                        max_count += 1;
                        flowerbed[0] = 1;
                    }
                } else if i == flowerbed.len() - 1 {
                    if flowerbed[i - 1] == 0 {
                        max_count += 1;
                        break;
                    }
                } else {
                    if flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0 {
                        max_count += 1;

                        flowerbed[i] = 1;
                    }
                }
            }
        }

        max_count >= n
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    // 官方示例
    #[test]
    fn test_can_place_flowers_examples() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }

    // 两端为空地，首尾两块也可以种
    #[test]
    fn test_can_place_flowers_both_ends_empty() {
        // 官方用例
        assert!(Solution::can_place_flowers(vec![0, 0, 1, 0, 0], 1));
        // 该数组最多能种 2 朵（index 0 和 index 4），不能再多
        assert!(Solution::can_place_flowers(vec![0, 0, 1, 0, 0], 2));
        assert!(!Solution::can_place_flowers(vec![0, 0, 1, 0, 0], 3));
    }

    // 只有一块地
    #[test]
    fn test_can_place_flowers_single_plot() {
        assert!(Solution::can_place_flowers(vec![0], 1));
        assert!(!Solution::can_place_flowers(vec![1], 1));
    }

    // 连续空地时，种下的花不能相邻
    #[test]
    fn test_can_place_flowers_consecutive_empty() {
        assert!(Solution::can_place_flowers(vec![0, 0], 1));
        assert!(!Solution::can_place_flowers(vec![0, 0], 2));
        assert!(Solution::can_place_flowers(vec![0, 0, 0], 2));
        assert!(!Solution::can_place_flowers(vec![0, 0, 0], 3));
        assert!(Solution::can_place_flowers(vec![0, 0, 0, 0], 2));
        assert!(!Solution::can_place_flowers(vec![0, 0, 0, 0], 3));
    }

    // n 为 0 时不需要种植
    #[test]
    fn test_can_place_flowers_none_needed() {
        assert!(Solution::can_place_flowers(vec![1], 0));
        assert!(Solution::can_place_flowers(vec![1, 1, 1], 0));
        assert!(Solution::can_place_flowers(vec![1, 0, 1], 0));
    }
}
