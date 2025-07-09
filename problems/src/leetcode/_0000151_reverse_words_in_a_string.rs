// 0000151. Reverse Words in a String
// https://leetcode.com/problems/reverse-words-in-a-string/description/
// Given an input string s, reverse the order of the words.
//
// A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.
//
// Return a string of the words in reverse order concatenated by a single space.
//
// Note that s may contain leading or trailing spaces or multiple spaces between two words. The returned string should only have a single space separating the words. Do not include any extra spaces.
//
// Example 1:
//
// Input: s = "the sky is blue"
// Output: "blue is sky the"
//
// Example 2:
//
// Input: s = "  hello world  "
// Output: "world hello"
// Explanation: Your reversed string should not contain leading or trailing spaces.
//
// Example 3:
//
// Input: s = "a good   example"
// Output: "example good a"
// Explanation: You need to reduce multiple spaces between two words to a single space in the reversed string.
//
// Constraints:
//
//     1 <= s.length <= 104
//     s contains English letters (upper-case and lower-case), digits, and spaces ' '.
//     There is at least one word in s.
//
// Follow-up: If the string data type is mutable in your language, can you solve it in-place with O(1) extra space?
//
// ---
pub fn reverse_words(s: String) -> String {
    s.split_ascii_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}
// ---

pub fn testcase() {
    let s = String::from("the sky is blue");
    let res = reverse_words(s);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("the sky is blue");
        let res = reverse_words(s);
        let expected = String::from("blue is sky the");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let s = String::from("  hello world  ");
        let res = reverse_words(s);
        let expected = String::from("world hello");
        assert_eq!(
            res, expected,
            "Your reversed string should not contain leading or trailing spaces."
        );
    }

    #[test]
    fn test_3() {
        let s = String::from("a good   example");
        let res = reverse_words(s);
        let expected = String::from("example good a");
        assert_eq!(
            res, expected,
            "You need to reduce multiple spaces between two words to a single space in the reversed string."
        );
    }
}
