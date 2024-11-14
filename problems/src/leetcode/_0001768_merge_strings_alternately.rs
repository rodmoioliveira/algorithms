// 0001768. Merge Strings Alternately
// https://leetcode.com/problems/merge-strings-alternately/description/
// You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.
//
// Return the merged string.
//
// Example 1:
//
// Input: word1 = "abc", word2 = "pqr"
// Output: "apbqcr"
// Explanation: The merged string will be merged as so:
// word1:  a   b   c
// word2:    p   q   r
// merged: a p b q c r
//
// Example 2:
//
// Input: word1 = "ab", word2 = "pqrs"
// Output: "apbqrs"
// Explanation: Notice that as word2 is longer, "rs" is appended to the end.
// word1:  a   b
// word2:    p   q   r   s
// merged: a p b q   r   s
//
// Example 3:
//
// Input: word1 = "abcd", word2 = "pq"
// Output: "apbqcd"
// Explanation: Notice that as word1 is longer, "cd" is appended to the end.
// word1:  a   b   c   d
// word2:    p   q
// merged: a p b q c   d
//
// Constraints:
//
//     1 <= word1.length, word2.length <= 100
//     word1 and word2 consist of lowercase English letters.
//
// ---
pub fn merge_alternately(word1: String, word2: String) -> String {
    let w1_len = word1.len();
    let w2_len = word2.len();
    let max_len = std::cmp::max(w1_len, w2_len);

    let w1_chars: Vec<char> = word1.chars().collect();
    let w2_chars: Vec<char> = word2.chars().collect();

    let mut s = String::with_capacity(w1_len + w2_len);
    for i in 0..max_len {
        if i < w1_len {
            s.push(w1_chars[i]);
        }
        if i < w2_len {
            s.push(w2_chars[i]);
        }
    }
    s
}
// ---

pub fn testcase() {
    let word1 = "ab".to_string();
    let word2 = "pqrs".to_string();
    let result = merge_alternately(word1, word2);
    eprintln!("0001768_merge_strings_alternately: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let word1 = "abc".to_string();
        let word2 = "pqr".to_string();
        let result = merge_alternately(word1, word2);
        let expected = String::from("apbqcr");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let word1 = "ab".to_string();
        let word2 = "pqrs".to_string();
        let result = merge_alternately(word1, word2);
        let expected = String::from("apbqrs");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let word1 = "abcd".to_string();
        let word2 = "pq".to_string();
        let result = merge_alternately(word1, word2);
        let expected = String::from("apbqcd");
        assert_eq!(result, expected);
    }
}
