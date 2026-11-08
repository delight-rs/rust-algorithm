// https://leetcode.com/problems/reverse-words-in-a-string/?envType=study-plan-v2&envId=leetcode-75

struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }

    pub fn reverse_words_2(s: String) -> String {
        let words: Vec<&str> = s.split_whitespace().collect();
        let mut result = String::with_capacity(s.len());

        for i in (0..words.len()).rev() {
            result += words[i];
            if i != 0 {
                result += " ";
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    // 官方示例
    #[test]
    fn test_reverse_words_example_1() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_string()),
            "blue is sky the"
        );
    }

    // 官方示例：前导和尾随空格需要去掉
    #[test]
    fn test_reverse_words_example_2() {
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_string()),
            "world hello"
        );
    }

    // 官方示例：单词间的多个空格压缩为一个
    #[test]
    fn test_reverse_words_example_3() {
        assert_eq!(
            Solution::reverse_words("a good   example".to_string()),
            "example good a"
        );
    }

    // 只有前导空格
    #[test]
    fn test_reverse_words_leading_spaces() {
        assert_eq!(
            Solution::reverse_words("   the sky is blue".to_string()),
            "blue is sky the"
        );
    }

    // 只有尾随空格
    #[test]
    fn test_reverse_words_trailing_spaces() {
        assert_eq!(
            Solution::reverse_words("the sky is blue   ".to_string()),
            "blue is sky the"
        );
    }

    // 单词之间以及两端都有多个空格
    #[test]
    fn test_reverse_words_multiple_spaces() {
        assert_eq!(
            Solution::reverse_words("  Bob    Loves  Alice   ".to_string()),
            "Alice Loves Bob"
        );
    }

    // 单个单词，前后无空格
    #[test]
    fn test_reverse_words_single_word() {
        assert_eq!(Solution::reverse_words("hello".to_string()), "hello");
    }

    // 单个单词，前后都有空格
    #[test]
    fn test_reverse_words_single_word_with_spaces() {
        assert_eq!(Solution::reverse_words("   hello   ".to_string()), "hello");
    }

    // 每个单词只有一个字符
    #[test]
    fn test_reverse_words_single_char_words() {
        assert_eq!(Solution::reverse_words("a b c".to_string()), "c b a");
    }

    // 单词包含数字
    #[test]
    fn test_reverse_words_with_digits() {
        assert_eq!(
            Solution::reverse_words("abc 123 xyz".to_string()),
            "xyz 123 abc"
        );
    }

    // 更长的句子，验证整体顺序反转且单词内部保持原样
    #[test]
    fn test_reverse_words_long_sentence() {
        assert_eq!(
            Solution::reverse_words("Alice does not even like bob".to_string()),
            "bob like even not does Alice"
        );
    }
}
