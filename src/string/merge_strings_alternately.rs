// https://leetcode.com/problems/merge-strings-alternately/description/?envType=study-plan-v2&envId=leetcode-75

use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn merge_alternately(mut word1: String, mut word2: String) -> String {
        let len = min(word1.len(), word2.len());
        let mut result = String::with_capacity(word1.len() + word2.len());

        for _ in 0..len {
            result.push(word1.remove(0));
            result.push(word2.remove(0));
        }

        if word1.is_empty() && !word2.is_empty() {
            result += word2.as_str();
        }

        if word2.is_empty() && !word1.is_empty() {
            result += word1.as_str();
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_merge_alternately() {
        assert_eq!(
            Solution::merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr"
        );
    }

    #[test]
    fn test_merge_alternately_word2_longer() {
        assert_eq!(
            Solution::merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs"
        );
    }

    #[test]
    fn test_merge_alternately_word1_longer() {
        assert_eq!(
            Solution::merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd"
        );
    }

    #[test]
    fn test_merge_alternately_with_empty_word() {
        assert_eq!(
            Solution::merge_alternately(String::new(), "xyz".to_string()),
            "xyz"
        );
        assert_eq!(
            Solution::merge_alternately("xyz".to_string(), String::new()),
            "xyz"
        );
    }

    #[test]
    fn test_merge_alternately_both_empty() {
        assert_eq!(
            Solution::merge_alternately(String::new(), String::new()),
            ""
        );
    }

    #[test]
    fn test_merge_alternately_single_chars() {
        assert_eq!(
            Solution::merge_alternately("a".to_string(), "b".to_string()),
            "ab"
        );
    }
}
