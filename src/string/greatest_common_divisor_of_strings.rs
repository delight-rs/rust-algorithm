// https://leetcode.com/problems/greatest-common-divisor-of-strings/description/?envType=study-plan-v2&envId=leetcode-75

struct Solution {}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        todo!()
    }

    fn is_division(str: &str, d: &str) -> bool {
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
