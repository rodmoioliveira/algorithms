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
fn is_vowel(b: u8) -> bool {
    matches!(
        b,
        b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
    )
}

pub fn reverse_vowels(s: String) -> String {
    let mut b = s.as_bytes().to_vec();
    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        while left < right && !is_vowel(b[left]) {
            left += 1;
        }
        while right > 0 && !is_vowel(b[right]) {
            right -= 1;
        }
        if left < right {
            b.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    String::from_utf8(b).unwrap()
}
// ---

pub fn testcase() {
    let s = String::from("IceCreAm");
    let res = reverse_vowels(s);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("IceCreAm");
        let res = reverse_vowels(s);
        let expected = String::from("AceCreIm");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let s = String::from("leetcode");
        let res = reverse_vowels(s);
        let expected = String::from("leotcede");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let s = String::from(" ");
        let res = reverse_vowels(s);
        let expected = String::from(" ");
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let s = String::from(".,");
        let res = reverse_vowels(s);
        let expected = String::from(".,");
        assert_eq!(res, expected);
    }
}
