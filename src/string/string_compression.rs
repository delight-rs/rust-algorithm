// CN: https://leetcode.cn/problems/string-compression/description/
// EN: https://leetcode.com/problems/string-compression/?envType=study-plan-v2&envId=leetcode-75

struct Solution {}

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let len = chars.len();

        if len <= 1 {
            return len as i32;
        }

        let mut i = 0;
        let mut start = 0;
        let mut next = 1;

        while next < len {
            if chars[start] != chars[next] {
                Self::modify(chars, &mut i, start, next);
                start = next;
            }

            next += 1;
        }

        Self::modify(chars, &mut i, start, next);

        i as i32
    }

    fn modify(chars: &mut Vec<char>, i: &mut usize, start: usize, next: usize) {
        chars[*i] = chars[start];
        *i += 1;

        let count = next - start;
        if count >= 2 {
            for c in count.to_string().chars() {
                chars[*i] = c;
                *i += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_compress_example1() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let len = Solution::compress(&mut chars) as usize;
        assert_eq!(len, 6);
        assert_eq!(chars[..len].iter().copied().collect::<String>(), "a2b2c3");
    }

    #[test]
    fn test_compress_example2_single_char() {
        let mut chars = vec!['a'];
        let len = Solution::compress(&mut chars) as usize;
        assert_eq!(len, 1);
        assert_eq!(chars[..len].iter().copied().collect::<String>(), "a");
    }

    #[test]
    fn test_compress_example3() {
        let mut chars = vec!['a'];
        chars.extend(vec!['b'; 12]);
        let len = Solution::compress(&mut chars) as usize;
        assert_eq!(len, 4);
        assert_eq!(chars[..len].iter().copied().collect::<String>(), "ab12");
    }

    #[test]
    fn test_compress_all_distinct() {
        let mut chars = vec!['a', 'b', 'c'];
        let len = Solution::compress(&mut chars) as usize;
        assert_eq!(len, 3);
        assert_eq!(chars[..len].iter().copied().collect::<String>(), "abc");
    }

    #[test]
    fn test_compress_group_len_10() {
        let mut chars = vec!['a'; 10];
        let len = Solution::compress(&mut chars) as usize;
        assert_eq!(len, 3);
        assert_eq!(chars[..len].iter().copied().collect::<String>(), "a10");
    }

    #[test]
    fn test_compress_group_len_11() {
        let mut chars = vec!['a'; 11];
        let len = Solution::compress(&mut chars) as usize;
        assert_eq!(len, 3);
        assert_eq!(chars[..len].iter().copied().collect::<String>(), "a11");
    }

    #[test]
    fn test_compress_two_multi_digit_groups() {
        let mut chars = vec!['a'; 12];
        chars.extend(vec!['b'; 10]);
        let len = Solution::compress(&mut chars) as usize;
        assert_eq!(len, 6);
        assert_eq!(chars[..len].iter().copied().collect::<String>(), "a12b10");
    }

    #[test]
    fn test_compress_max_length_same_char() {
        let mut chars = vec!['x'; 2000];
        let len = Solution::compress(&mut chars) as usize;
        assert_eq!(len, 5);
        assert_eq!(chars[..len].iter().copied().collect::<String>(), "x2000");
    }

    #[test]
    fn test_compress_digit_chars() {
        let mut chars = vec!['1', '1', '2'];
        let len = Solution::compress(&mut chars) as usize;
        assert_eq!(len, 3);
        assert_eq!(chars[..len].iter().copied().collect::<String>(), "122");
    }

    #[test]
    fn test_compress_mixed_case() {
        let mut chars = vec!['A', 'A', 'a'];
        let len = Solution::compress(&mut chars) as usize;
        assert_eq!(len, 3);
        assert_eq!(chars[..len].iter().copied().collect::<String>(), "A2a");
    }
}
