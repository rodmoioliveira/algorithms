// 0000125. Valid Palindrome
// https://leetcode.com/problems/valid-palindrome/description/
// A phrase is a palindrome if it reads the same forward and backward.
//
// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and
// removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric
// characters include letters and numbers.
//
// Given a string s, return true if it is a palindrome, or false otherwise.
//
// Example 1:
//
// Input: s = "A man, a plan, a canal: Panama"
// Output: true
// Explanation: "amanaplanacanalpanama" is a palindrome.
//
// Example 2:
//
// Input: s = "race a car"
// Output: false
// Explanation: "raceacar" is not a palindrome.
//
// Example 3:
//
// Input: s = " "
// Output: true
// Explanation: s is an empty string "" after removing non-alphanumeric characters.
// Since an empty string reads the same forward and backward, it is a palindrome.
//
// Constraints:
//
//     1 <= s.length <= 2 * 105
//     s consists only of printable ASCII characters.
// ---
pub fn is_palindrome(s: String) -> bool {
    let p: Vec<char> = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let mut v = p.clone();
    v.reverse();
    p == v
}
// ---

pub fn testcase() {
    let res = is_palindrome(String::from("A man, a plan, a canal: Panama"));
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let res = is_palindrome(String::from("A man, a plan, a canal: Panama"));
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let res = is_palindrome(String::from("race a car"));
        let expected = false;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let res = is_palindrome(String::from(" "));
        let expected = true;
        assert_eq!(res, expected);
    }
}
