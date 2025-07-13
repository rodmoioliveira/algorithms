// 0000020. Valid Parentheses
// https://leetcode.com/problems/valid-parentheses/description/
// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//
// An input string is valid if:
//
//     Open brackets must be closed by the same type of brackets.
//     Open brackets must be closed in the correct order.
//     Every close bracket has a corresponding open bracket of the same type.
//
// Example 1:
// Input: s = "()"
// Output: true
//
// Example 2:
// Input: s = "()[]{}"
// Output: true
//
// Example 3:
// Input: s = "(]"
// Output: false
//
// Example 4:
// Input: s = "([])"
// Output: true
//
// Constraints:
//
//     1 <= s.length <= 104
//     s consists of parentheses only '()[]{}'.
// ---
pub fn is_valid(s: String) -> bool {
    use std::collections::HashMap;

    let mut v = Vec::with_capacity(s.len());
    let m: HashMap<char, char> = HashMap::from([(']', '['), (')', '('), ('}', '{')]);

    for c in s.chars() {
        match c {
            ']' | ')' | '}' => {
                if m.get(&c).cloned() != v.pop() {
                    return false;
                }
            }
            c => v.push(c),
        }
    }

    v.is_empty()
}
// ---

pub fn testcase() {
    let result = is_valid(String::from("()"));
    eprintln!("{} {:?}", module_path!(), result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = is_valid(String::from("()"));
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let result = is_valid(String::from("()[]{}"));
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let result = is_valid(String::from("(]"));
        let expected = false;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_4() {
        let result = is_valid(String::from("([])"));
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_5() {
        let result = is_valid(String::from("]"));
        let expected = false;
        assert_eq!(result, expected);
    }
}
