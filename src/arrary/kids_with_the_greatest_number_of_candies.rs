// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/description/?envType=study-plan-v2&envId=leetcode-75

struct Solution {}

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut max = candies[0];

        for i in 1..candies.len() {
            if candies[i] > max {
                max = candies[i]
            }
        }

        let mut result = Vec::with_capacity(candies.len());
        for i in 0..candies.len() {
            if candies[i] + extra_candies >= max {
                result.push(true);
            } else {
                result.push(false);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_kids_with_candies_example_1() {
        assert_eq!(
            Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
    }

    #[test]
    fn test_kids_with_candies_example_2() {
        assert_eq!(
            Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
    }

    #[test]
    fn test_kids_with_candies_example_3() {
        assert_eq!(
            Solution::kids_with_candies(vec![12, 1, 12], 10),
            vec![true, false, true]
        );
    }

    #[test]
    fn test_kids_with_candies_single_kid() {
        assert_eq!(Solution::kids_with_candies(vec![5], 0), vec![true]);

        assert_eq!(Solution::kids_with_candies(vec![5], 3), vec![true]);
    }

    #[test]
    fn test_kids_with_candies_all_equal() {
        assert_eq!(
            Solution::kids_with_candies(vec![1, 1, 1, 1], 0),
            vec![true, true, true, true]
        );
    }

    #[test]
    fn test_kids_with_candies_zero_extra_candies() {
        assert_eq!(
            Solution::kids_with_candies(vec![1, 2, 3], 0),
            vec![false, false, true]
        );
    }

    #[test]
    fn test_kids_with_candies_large_extra_candies() {
        assert_eq!(
            Solution::kids_with_candies(vec![1, 2, 3], 100),
            vec![true, true, true]
        );
    }
}
