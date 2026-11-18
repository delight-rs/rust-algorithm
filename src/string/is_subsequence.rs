// CN: https://leetcode.cn/problems/is-subsequence/description/
// EN: https://leetcode.com/problems/is-subsequence/?envType=study-plan-v2&envId=leetcode-75

struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let mut i = 0;
        let s_bytes = s.as_bytes();

        for b in t.as_bytes() {
            if s_bytes.get(i) == Some(b) {
                i += 1;
            }
        }

        i == s_bytes.len()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_is_subsequence_example_1() {
        assert!(Solution::is_subsequence(
            "abc".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn test_is_subsequence_example_2() {
        assert!(!Solution::is_subsequence(
            "axc".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn test_is_subsequence_both_empty() {
        assert!(Solution::is_subsequence("".to_string(), "".to_string()));
    }

    #[test]
    fn test_is_subsequence_empty_s() {
        assert!(Solution::is_subsequence(
            "".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn test_is_subsequence_empty_t() {
        assert!(!Solution::is_subsequence("abc".to_string(), "".to_string()));
    }

    #[test]
    fn test_is_subsequence_equal_strings() {
        assert!(Solution::is_subsequence(
            "abc".to_string(),
            "abc".to_string()
        ));
    }

    #[test]
    fn test_is_subsequence_s_longer_than_t() {
        assert!(!Solution::is_subsequence(
            "abcd".to_string(),
            "abc".to_string()
        ));
    }

    #[test]
    fn test_is_subsequence_last_char_match() {
        assert!(Solution::is_subsequence("c".to_string(), "abc".to_string()));
    }

    #[test]
    fn test_is_subsequence_order_violated() {
        assert!(!Solution::is_subsequence(
            "aec".to_string(),
            "abcde".to_string()
        ));
    }

    #[test]
    fn test_is_subsequence_non_contiguous() {
        assert!(Solution::is_subsequence(
            "ace".to_string(),
            "abcde".to_string()
        ));
    }

    #[test]
    fn test_is_subsequence_duplicate_chars() {
        assert!(Solution::is_subsequence(
            "aba".to_string(),
            "abba".to_string()
        ));
    }

    #[test]
    fn test_is_subsequence_repeated_single_char_not_enough() {
        assert!(!Solution::is_subsequence(
            "aaa".to_string(),
            "aa".to_string()
        ));
    }

    #[test]
    fn test_is_subsequence_max_length() {
        assert!(Solution::is_subsequence(
            "a".repeat(100),
            "a".repeat(10_000)
        ));
    }
}
