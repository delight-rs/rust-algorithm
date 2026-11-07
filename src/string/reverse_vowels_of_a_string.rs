// https://leetcode.com/problems/reverse-vowels-of-a-string/description/?envType=study-plan-v2&envId=leetcode-75

struct Solution {}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();

        let mut left = 0;
        let mut right = chars.len() - 1;

        while left < right {
            if !Solution::is_vowel(chars[left]) {
                left += 1;
            } else if !Solution::is_vowel(chars[right]) {
                right -= 1;
            } else {
                chars.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        String::from_iter(chars)
    }

    fn is_vowel(c: char) -> bool {
        let c = c.to_ascii_lowercase();
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::reverse_vowels("IceCreAm".to_string()), "AceCreIm");
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede");
    }

    #[test]
    fn single_vowel() {
        assert_eq!(Solution::reverse_vowels("a".to_string()), "a");
        assert_eq!(Solution::reverse_vowels("A".to_string()), "A");
    }

    #[test]
    fn single_consonant() {
        assert_eq!(Solution::reverse_vowels("b".to_string()), "b");
        assert_eq!(Solution::reverse_vowels("Z".to_string()), "Z");
    }

    #[test]
    fn no_vowels() {
        assert_eq!(Solution::reverse_vowels("bcdfg".to_string()), "bcdfg");
    }

    #[test]
    fn all_vowels() {
        assert_eq!(Solution::reverse_vowels("aeiou".to_string()), "uoiea");
        assert_eq!(Solution::reverse_vowels("AEIOU".to_string()), "UOIEA");
    }

    #[test]
    fn odd_number_of_vowels() {
        assert_eq!(Solution::reverse_vowels("aei".to_string()), "iea");
    }

    #[test]
    fn mixed_case_vowels() {
        assert_eq!(Solution::reverse_vowels("aA".to_string()), "Aa");
        assert_eq!(Solution::reverse_vowels("hEllo".to_string()), "hollE");
    }

    #[test]
    fn vowels_at_both_ends() {
        assert_eq!(Solution::reverse_vowels("abcu".to_string()), "ubca");
    }

    #[test]
    fn duplicated_vowels() {
        assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle");
    }

    #[test]
    fn already_palindromic_vowels() {
        assert_eq!(Solution::reverse_vowels("racecar".to_string()), "racecar");
    }

    #[test]
    fn printable_ascii_non_letters() {
        assert_eq!(Solution::reverse_vowels(".,!? 123".to_string()), ".,!? 123");
    }

    #[test]
    fn mixed_digits_and_vowels() {
        assert_eq!(Solution::reverse_vowels("1a2e3".to_string()), "1e2a3");
    }

    #[test]
    fn is_vowel_accepts_lowercase_and_uppercase() {
        for c in ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'] {
            assert!(Solution::is_vowel(c), "expected {c:?} to be a vowel");
        }
    }

    #[test]
    fn is_vowel_rejects_consonants_and_non_letters() {
        for c in ['b', 'y', 'Z', '1', ' ', '.', '\n'] {
            assert!(!Solution::is_vowel(c), "expected {c:?} not to be a vowel");
        }
    }
}
