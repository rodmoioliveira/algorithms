// 0000345. Reverse Vowels of a String
// https://leetcode.com/problems/reverse-vowels-of-a-string/description/
// Given a string s, reverse only all the vowels in the string and return it.
//
// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.
//
// Example 1:
//
// Input: s = "IceCreAm"
// Output: "AceCreIm"
// Explanation:
// The vowels in s are ['I', 'e', 'e', 'A']. On reversing the vowels, s becomes "AceCreIm".
//
// Example 2:
//
// Input: s = "leetcode"
// Output: "leotcede"
// Constraints:
//
//     1 <= s.length <= 3 * 105
//     s consist of printable ASCII characters.
//
// ---
pub fn reverse_vowels(s: String) -> String {
    use std::char;
    use std::collections::HashSet;

    let s_len = s.len();
    if s_len < 2 {
        return s;
    }

    let vowels_set: HashSet<char> = HashSet::from_iter("aeiou".chars());
    let mut chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = s_len - 1;

    loop {
        while left < right && !vowels_set.contains(&chars[left].to_ascii_lowercase()) {
            left += 1;
        }
        while right > 0 && !vowels_set.contains(&chars[right].to_ascii_lowercase()) {
            right -= 1;
        }
        if left >= right {
            break;
        }

        if left != right {
            chars.swap(left, right);
        }
        left += 1;
        right -= 1;
    }

    chars.into_iter().collect()
}
// ---

pub fn testcase() {
    let s = String::from("IceCreAm");
    let result = reverse_vowels(s);
    eprintln!("leetcode/0000345_reverse_vowels_of_a_string: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("IceCreAm");
        let result = reverse_vowels(s);
        let expected = String::from("AceCreIm");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let s = String::from("leetcode");
        let result = reverse_vowels(s);
        let expected = String::from("leotcede");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let s = String::from(" ");
        let result = reverse_vowels(s);
        let expected = String::from(" ");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_4() {
        let s = String::from(".,");
        let result = reverse_vowels(s);
        let expected = String::from(".,");
        assert_eq!(result, expected);
    }
}
