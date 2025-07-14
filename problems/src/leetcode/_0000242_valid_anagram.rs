// 0000242. Valid Anagram
// https://leetcode.com/problems/valid-anagram/description/
// Given two strings s and t, return true if t is an of s, and false otherwise.
//
// Example 1:
//
// Input: s = "anagram", t = "nagaram"
//
// Output: true
//
// Example 2:
//
// Input: s = "rat", t = "car"
//
// Output: false
//
// Constraints:
//
//     1 <= s.length, t.length <= 5 * 104
//     s and t consist of lowercase English letters.
//
// Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?
// ---
pub fn is_anagram(s: String, t: String) -> bool {
    let mut _s: Vec<char> = s.chars().collect();
    let mut _t: Vec<char> = t.chars().collect();

    _s.sort();
    _t.sort();

    _s == _t
}
// ---

pub fn testcase() {
    let res = is_anagram(String::from("anagram"), String::from("nagaram"));
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let res = is_anagram(String::from("anagram"), String::from("nagaram"));
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let res = is_anagram(String::from("rat"), String::from("car"));
        let expected = false;
        assert_eq!(res, expected);
    }
}
