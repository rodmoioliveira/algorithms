// 0000394. Decode String
// https://leetcode.com/problems/decode-string/description/
// Given an encoded string, return its decoded string.
//
// The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is
// being repeated exactly k times. Note that k is guaranteed to be a positive integer.
//
// You may assume that the input string is always valid; there are no extra white spaces, square
// brackets are well-formed, etc. Furthermore, you may assume that the original data does not
// contain any digits and that digits are only for those repeat numbers, k. For example, there will
// not be input like 3a or 2[4].
//
// The test cases are generated so that the length of the output will never exceed 105.
//
// Example 1:
//
// Input: s = "3[a]2[bc]"
// Output: "aaabcbc"
//
// Example 2:
//
// Input: s = "3[a2[c]]"
// Output: "accaccacc"
//
// Example 3:
//
// Input: s = "2[abc]3[cd]ef"
// Output: "abcabccdcdcdef"
//
// Constraints:
//
//     1 <= s.length <= 30
//     s consists of lowercase English letters, digits, and square brackets '[]'.
//     s is guaranteed to be a valid input.
//     All the integers in s are in the range [1, 300].
// ---
pub fn decode_string(s: String) -> String {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            ']' => {
                let mut alpha = Vec::new();
                let mut digits = Vec::new();

                while let Some(v) = stack.last() {
                    if v.is_ascii_punctuation() {
                        let _ = stack.pop();
                    } else if v.is_alphabetic() {
                        alpha.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }

                while let Some(v) = stack.last() {
                    if v.is_ascii_digit() {
                        digits.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }

                alpha.reverse();
                digits.reverse();

                let n = String::from_iter(digits)
                    .parse::<usize>()
                    .unwrap_or_default();
                for c in String::from_iter(alpha).repeat(n).chars() {
                    stack.push(c);
                }
            }
            _ => {
                stack.push(c);
            }
        }
    }
    String::from_iter(stack)
}

pub fn decode_string_vecdeque(s: String) -> String {
    use std::collections::VecDeque;

    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            ']' => {
                let mut alpha = VecDeque::new();
                let mut digits = VecDeque::new();

                while let Some(v) = stack.last() {
                    if v.is_ascii_punctuation() {
                        let _ = stack.pop();
                    } else if v.is_alphabetic() {
                        alpha.push_front(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }

                while let Some(v) = stack.last() {
                    if v.is_ascii_digit() {
                        digits.push_front(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }

                let n = String::from_iter(digits)
                    .parse::<usize>()
                    .unwrap_or_default();
                for c in String::from_iter(alpha).repeat(n).chars() {
                    stack.push(c);
                }
            }
            _ => {
                stack.push(c);
            }
        }
    }
    String::from_iter(stack)
}

pub fn decode_string_pop_if(s: String) -> String {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            ']' => {
                let mut alpha = Vec::new();
                let mut digits = Vec::new();

                while let Some(l) = stack.pop_if(|s| s.is_ascii_punctuation() || s.is_alphabetic())
                {
                    if l.is_alphabetic() {
                        alpha.push(l);
                    }
                }

                while let Some(l) = stack.pop_if(|s| s.is_ascii_digit()) {
                    digits.push(l);
                }

                alpha.reverse();
                digits.reverse();

                let n = String::from_iter(digits)
                    .parse::<usize>()
                    .unwrap_or_default();
                for c in String::from_iter(alpha).repeat(n).chars() {
                    stack.push(c);
                }
            }
            _ => {
                stack.push(c);
            }
        }
    }
    String::from_iter(stack)
}

pub fn decode_string_pop_if_vecdeque(s: String) -> String {
    use std::collections::VecDeque;

    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            ']' => {
                let mut alpha = VecDeque::new();
                let mut digits = VecDeque::new();

                while let Some(l) = stack.pop_if(|s| s.is_ascii_punctuation() || s.is_alphabetic())
                {
                    if l.is_alphabetic() {
                        alpha.push_front(l);
                    }
                }

                while let Some(l) = stack.pop_if(|s| s.is_ascii_digit()) {
                    digits.push_front(l);
                }

                let n = String::from_iter(digits)
                    .parse::<usize>()
                    .unwrap_or_default();
                for c in String::from_iter(alpha).repeat(n).chars() {
                    stack.push(c);
                }
            }
            _ => {
                stack.push(c);
            }
        }
    }
    String::from_iter(stack)
}
// ---

pub fn testcase() {
    let s = String::from("3[z]2[2[y]pq4[2[jk]e1[f]]]ef");
    let res = decode_string(s);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("3[a]2[bc]");
        let res = decode_string(s);
        assert_eq!(res, String::from("aaabcbc"));
    }

    #[test]
    fn test_2() {
        let s = String::from("3[a2[c]]");
        let res = decode_string(s);
        assert_eq!(res, String::from("accaccacc"));
    }

    #[test]
    fn test_3() {
        let s = String::from("2[abc]3[cd]ef");
        let res = decode_string(s);
        assert_eq!(res, String::from("abcabccdcdcdef"));
    }

    #[test]
    fn test_4() {
        let s = String::from("100[leetcode]");
        let res = decode_string(s);
        assert_eq!(
            res,
            String::from(
                "leetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcode"
            )
        );
    }

    #[test]
    fn test_5() {
        let s = String::from("3[z]2[2[y]pq4[2[jk]e1[f]]]ef");
        let res = decode_string(s);
        assert_eq!(
            res,
            String::from("zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef")
        );
    }
}

#[cfg(test)]
mod tests_vecdeque {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("3[a]2[bc]");
        let res = decode_string_vecdeque(s);
        assert_eq!(res, String::from("aaabcbc"));
    }

    #[test]
    fn test_2() {
        let s = String::from("3[a2[c]]");
        let res = decode_string_vecdeque(s);
        assert_eq!(res, String::from("accaccacc"));
    }

    #[test]
    fn test_3() {
        let s = String::from("2[abc]3[cd]ef");
        let res = decode_string_vecdeque(s);
        assert_eq!(res, String::from("abcabccdcdcdef"));
    }

    #[test]
    fn test_4() {
        let s = String::from("100[leetcode]");
        let res = decode_string_vecdeque(s);
        assert_eq!(
            res,
            String::from(
                "leetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcode"
            )
        );
    }

    #[test]
    fn test_5() {
        let s = String::from("3[z]2[2[y]pq4[2[jk]e1[f]]]ef");
        let res = decode_string_vecdeque(s);
        assert_eq!(
            res,
            String::from("zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef")
        );
    }
}

#[cfg(test)]
mod tests_pop_if {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("3[a]2[bc]");
        let res = decode_string_pop_if(s);
        assert_eq!(res, String::from("aaabcbc"));
    }

    #[test]
    fn test_2() {
        let s = String::from("3[a2[c]]");
        let res = decode_string_pop_if(s);
        assert_eq!(res, String::from("accaccacc"));
    }

    #[test]
    fn test_3() {
        let s = String::from("2[abc]3[cd]ef");
        let res = decode_string_pop_if(s);
        assert_eq!(res, String::from("abcabccdcdcdef"));
    }

    #[test]
    fn test_4() {
        let s = String::from("100[leetcode]");
        let res = decode_string_pop_if(s);
        assert_eq!(
            res,
            String::from(
                "leetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcode"
            )
        );
    }

    #[test]
    fn test_5() {
        let s = String::from("3[z]2[2[y]pq4[2[jk]e1[f]]]ef");
        let res = decode_string_pop_if(s);
        assert_eq!(
            res,
            String::from("zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef")
        );
    }
}

#[cfg(test)]
mod tests_pop_if_vecdeque {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("3[a]2[bc]");
        let res = decode_string_pop_if_vecdeque(s);
        assert_eq!(res, String::from("aaabcbc"));
    }

    #[test]
    fn test_2() {
        let s = String::from("3[a2[c]]");
        let res = decode_string_pop_if_vecdeque(s);
        assert_eq!(res, String::from("accaccacc"));
    }

    #[test]
    fn test_3() {
        let s = String::from("2[abc]3[cd]ef");
        let res = decode_string_pop_if_vecdeque(s);
        assert_eq!(res, String::from("abcabccdcdcdef"));
    }

    #[test]
    fn test_4() {
        let s = String::from("100[leetcode]");
        let res = decode_string_pop_if_vecdeque(s);
        assert_eq!(
            res,
            String::from(
                "leetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcode"
            )
        );
    }

    #[test]
    fn test_5() {
        let s = String::from("3[z]2[2[y]pq4[2[jk]e1[f]]]ef");
        let res = decode_string_pop_if_vecdeque(s);
        assert_eq!(
            res,
            String::from("zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef")
        );
    }
}
