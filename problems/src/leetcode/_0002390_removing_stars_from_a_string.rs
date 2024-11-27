// 0002390. Removing Stars From a String
// https://leetcode.com/problems/removing-stars-from-a-string/description/
// You are given a string s, which contains stars *.
//
// In one operation, you can:
//
// - Choose a star in s.
// - Remove the closest non-star character to its left, as well as remove the star itself.
//
// Return the string after all stars have been removed.
//
// Note:
//
//     The input will be generated such that the operation is always possible.
//     It can be shown that the resing string will always be unique.
//
// Example 1:
//
// Input: s = "leet**cod*e"
// Output: "lecoe"
// Explanation: Performing the removals from left to right:
// - The closest character to the 1st star is 't' in "leet**cod*e". s becomes "lee*cod*e".
// - The closest character to the 2nd star is 'e' in "lee*cod*e". s becomes "lecod*e".
// - The closest character to the 3rd star is 'd' in "lecod*e". s becomes "lecoe".
// There are no more stars, so we return "lecoe".
//
// Example 2:
//
// Input: s = "erase*****"
// Output: ""
// Explanation: The entire string is removed, so we return an empty string.
//
// Constraints:
//
//     1 <= s.length <= 105
//     s consists of lowercase English letters and stars *.
//     The operation above can be performed on s.
//
// ---
pub fn remove_stars(s: String) -> String {
    let mut stack: String = String::with_capacity(s.len());
    for c in s.chars() {
        if c == '*' {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    stack
}
// ---

pub fn testcase() {
    let s = String::from("leet**cod*e");
    let res = remove_stars(s);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("leet**cod*e");
        let res = remove_stars(s);
        let expected = String::from("lecoe");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let s = String::from("erase*****");
        let res = remove_stars(s);
        let expected = String::from("");
        assert_eq!(res, expected);
    }
}
