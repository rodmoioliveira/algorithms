// 0000392. Is Subsequence
// https://leetcode.com/problems/is-subsequence/description/
// Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
//
// A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).
//
// Example 1:
//
// Input: s = "abc", t = "ahbgdc"
// Output: true
//
// Example 2:
//
// Input: s = "axc", t = "ahbgdc"
// Output: false
//
// Constraints:
//
//     0 <= s.length <= 100
//     0 <= t.length <= 104
//     s and t consist only of lowercase English letters.
//
// Follow up: Suppose there are lots of incoming s, say s1, s2, ..., sk where k >= 109, and you want to check one by one to see if t has its subsequence. In this scenario, how would you change your code?
// ======================
// Previous Approaches
// ======================
pub fn is_subsequence_previous_approach_1(s: String, t: String) -> bool {
    let s_len = s.len();
    if s_len == 0 {
        return true;
    }

    let s_chars: Vec<char> = s.chars().collect();
    let s_last_index = s_len - 1;
    let mut j = 0;

    for c in t.chars() {
        if c == s_chars[j] {
            j += 1;
        }

        if j > s_last_index {
            return true;
        }
    }
    false
}
// ---
pub fn is_subsequence(s: String, t: String) -> bool {
    let s_len = s.len();
    let t_len = t.len();
    let s_char: Vec<char> = s.chars().collect();
    let t_char: Vec<char> = t.chars().collect();
    let mut s_idx = 0;
    let mut t_idx = 0;

    while s_idx < s_len && t_idx < t_len {
        if s_char[s_idx] == t_char[t_idx] {
            s_idx += 1;
        }
        t_idx += 1;
    }
    s_idx == s_len
}
// ---

pub fn testcase() {
    let s = String::from("abc");
    let t = String::from("ahbgdc");
    let res = is_subsequence(s, t);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        let res = is_subsequence(s, t);
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let s = String::from("axc");
        let t = String::from("ahbgdc");
        let res = is_subsequence(s, t);
        let expected = false;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let s = String::from("abc");
        let t = String::from("abc");
        let res = is_subsequence(s, t);
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let s = String::from("");
        let t = String::from("abc");
        let res = is_subsequence(s, t);
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_5() {
        let s = String::from("b");
        let t = String::from("c");
        let res = is_subsequence(s, t);
        let expected = false;
        assert_eq!(res, expected);
    }
}
