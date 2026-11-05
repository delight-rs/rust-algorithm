// https://leetcode.com/problems/greatest-common-divisor-of-strings/description/?envType=study-plan-v2&envId=leetcode-75

use std::mem;

struct Solution {}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.is_empty() || str2.is_empty() {
            return String::new();
        }

        if !str1
            .bytes()
            .chain(str2.bytes())
            .eq(str2.bytes().chain(str1.bytes()))
        {
            return String::new();
        }

        let (mut a, mut b) = (str1.len(), str2.len());

        while b != 0 {
            (a, b) = (b, a % b);
        }

        str1[..a].to_string()
    }

    pub fn gcd_of_strings_3(str1: String, str2: String) -> String {
        let (m, n) = (str1.len(), str2.len());

        for pre_len in (1..=m.min(n)).rev() {
            if m % pre_len != 0 || n % pre_len != 0 {
                continue;
            }

            let candidate = &str1[..pre_len];

            if str1 == candidate.repeat(m / pre_len) && str2 == candidate.repeat(n / pre_len) {
                return candidate.to_string();
            }
        }

        String::new()
    }

    pub fn gcd_of_strings_2(str1: String, str2: String) -> String {
        let (str1_len, str2_len) = (str1.len(), str2.len());
        let mut s = str1;
        let mut g = str2;

        if str1_len > str2_len {
            mem::swap(&mut s, &mut g);
        }

        let mut a = g.len();
        let mut b = s.len();

        while b != 0 {
            (a, b) = (b, a % b);
        }

        // `a` is the gcd of both lengths; the answer must be this prefix of `s`.
        let d = match s.get(..a) {
            Some(d) => d,
            None => return "".to_string(),
        };

        return if Solution::is_division(&s, d) && Solution::is_division(&g, d) {
            d.to_string()
        } else {
            "".to_string()
        };
    }

    fn is_division(str: &str, d: &str) -> bool {
        !d.is_empty()
            && str.len() % d.len() == 0
            && str
                .as_bytes()
                .chunks_exact(d.len())
                .all(|block| block == d.as_bytes())
    }

    fn is_division_3(str: &str, d: &str) -> bool {
        if str.is_empty() || d.is_empty() {
            return false;
        }

        if str.len() % d.len() != 0 {
            return false;
        };

        return str.eq(&d.repeat(str.len() / d.len()));
    }

    fn is_division_2(str: &str, d: &str) -> bool {
        let s_len = str.len();
        let d_len = d.len();

        if s_len % d_len != 0 {
            return false;
        }

        for i in 0..s_len {
            let j = i % d_len;

            match (str.get(i..i + 1), d.get(j..j + 1)) {
                (Some(x), Some(y)) => {
                    if x != y {
                        return false;
                    }
                }
                (_, _) => {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_gcd_of_strings_common_divisor() {
        assert_eq!(
            Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC"
        );
    }

    #[test]
    fn test_gcd_of_strings_reversed_arguments() {
        assert_eq!(
            Solution::gcd_of_strings("ABC".to_string(), "ABCABC".to_string()),
            "ABC"
        );
    }

    #[test]
    fn test_gcd_of_strings_partial_common_divisor() {
        assert_eq!(
            Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB"
        );
    }

    #[test]
    fn test_gcd_of_strings_identical_strings() {
        assert_eq!(
            Solution::gcd_of_strings("ABAB".to_string(), "ABAB".to_string()),
            "ABAB"
        );
    }

    #[test]
    fn test_gcd_of_strings_no_common_divisor() {
        assert_eq!(
            Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string()),
            ""
        );
        assert_eq!(
            Solution::gcd_of_strings("ABCDEF".to_string(), "ABC".to_string()),
            ""
        );
    }

    #[test]
    fn test_gcd_of_strings_shared_prefix_only() {
        assert_eq!(
            Solution::gcd_of_strings("XYZXYZ".to_string(), "ABCABCABC".to_string()),
            ""
        );
    }

    #[test]
    fn test_gcd_of_strings_repeated_pattern() {
        assert_eq!(
            Solution::gcd_of_strings(
                "TAUXXTAUXXTAUXXTAUXXTAUXX".to_string(),
                "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX".to_string(),
            ),
            "TAUXX"
        );
    }
}
