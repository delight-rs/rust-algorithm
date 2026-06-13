// 最长公共前缀
// https://leetcode.cn/problems/longest-common-prefix/description/

// 暴力遍历方法
pub fn longest_common_prefix_1(strs: Vec<String>) -> String {
    let mut prefix = String::with_capacity(20);

    if strs.len() <= 0 {
        return prefix;
    }

    // 选择第一个字符串进行后续比较
    let chars_strs: Vec<_> = strs.iter().map(|s| s.chars()).collect();
    let first = chars_strs.first().unwrap().clone();

    for (i, ch) in first.enumerate() {
        let mut is_prefix = true;

        for s in chars_strs.iter().skip(1) {
            match s.clone().nth(i) {
                Some(v) => {
                    if v != ch {
                        is_prefix = false
                    }
                }
                None => return prefix,
            }
        }

        if is_prefix {
            prefix.push(ch);
        } else {
            return prefix;
        }
    }

    return prefix;
}

// 按字典排序，比较第一个和最后一个两个最不相识的字符串
pub fn longest_common_prefix_2(mut strs: Vec<String>) -> String {
    if strs.len() <= 0 {
        return String::new();
    }

    if strs.len() == 1 {
        return strs[0].to_string();
    }

    strs.sort();
    let mut prefix = String::with_capacity(20);

    let first = strs.first().unwrap();
    let last = strs.last().unwrap();

    for i in 0..first.len() {
        let first_ch = &first[i..i + 1];
        let last_ch = &last[i..i + 1];
        if first_ch == last_ch {
            prefix.push_str(first_ch);
        } else {
            return prefix;
        }
    }

    return prefix;
}

#[cfg(test)]
mod test {
    use super::longest_common_prefix_1;
    use super::longest_common_prefix_2;

    #[test]
    fn test_longest_common_prefix_1() {
        let strs_1 = ["flower", "flow", "flight"].map(|s| s.to_string()).to_vec();
        let strs_2 = ["dog", "racecar", "car"].map(|s| s.to_string()).to_vec();
        let strs_3 = ["cir", "car"].map(|s| s.to_string()).to_vec();

        assert_eq!(longest_common_prefix_1(strs_1), "fl".to_string());
        assert_eq!(longest_common_prefix_1(strs_2), String::new());
        assert_eq!(longest_common_prefix_1(strs_3), "c".to_string());
    }

    #[test]
    fn test_longest_common_prefix_2() {
        let strs_1 = ["flower", "flow", "flight"].map(|s| s.to_string()).to_vec();
        let strs_2 = ["dog", "racecar", "car"].map(|s| s.to_string()).to_vec();
        let strs_3 = ["cir", "car"].map(|s| s.to_string()).to_vec();

        assert_eq!(longest_common_prefix_2(strs_1), "fl".to_string());
        assert_eq!(longest_common_prefix_2(strs_2), String::new());
        assert_eq!(longest_common_prefix_2(strs_3), "c".to_string());
    }
}
