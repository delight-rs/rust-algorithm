//! 汇总区间
//! https://leetcode.cn/problems/summary-ranges/description/?envType=problem-list-v2&envId=xulyhlrg

struct Solution;

impl Solution {
    pub fn summary_ranges(mut nums: Vec<i32>) -> Vec<String> {
        if nums.len() <= 1 {
            return nums.clone().iter().map(|n| n.to_string()).collect();
        }

        let mut result = vec![];
        let mut i = 0;
        let mut next = 1;

        let length = nums.len();

        while next < length {
            let step = nums[next] - nums[next - 1];

            if step != 1 {
                result.push(get_str(&nums, i, next));
                i = next;
            }

            next += 1;
        }

        result.push(get_str(&nums, i, next));
        result
    }
}

fn get_str(nums: &Vec<i32>, i: usize, next: usize) -> String {
    if i == next - 1 {
        nums[i].to_string()
    } else {
        format!("{}->{}", nums[i], nums[next - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_ranges() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        );

        assert_eq!(Solution::summary_ranges(vec![0, 1]), vec!["0->1"]);
    }
}
