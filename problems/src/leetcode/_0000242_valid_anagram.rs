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
pub fn is_anagram_sort(s: String, t: String) -> bool {
    let mut _s: Vec<char> = s.chars().collect();
    let mut _t: Vec<char> = t.chars().collect();

    _s.sort();
    _t.sort();

    _s == _t
}

pub fn is_anagram_frequency(s: String, t: String) -> bool {
    let mut f: [i32; 26] = [0; 26];
    for b in s.bytes() {
        f[(b - b'a') as usize] += 1;
    }
    for b in t.bytes() {
        f[(b - b'a') as usize] -= 1;
    }

    f.iter().all(|x| *x == 0)
}

pub fn is_anagram_hashmap(s: String, t: String) -> bool {
    use std::collections::HashMap;

    let mut f: HashMap<char, i32> = HashMap::new();
    for b in s.chars() {
        f.entry(b).and_modify(|e| *e += 1).or_insert(1);
    }
    for b in t.chars() {
        f.entry(b).and_modify(|e| *e -= 1).or_insert(-1);
    }

    f.values().all(|x| *x == 0)
}
// ---

pub fn testcase() {
    let res = is_anagram_sort(String::from("anagram"), String::from("nagaram"));
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests_sort {
    use super::*;

    #[test]
    fn test_1() {
        let res = is_anagram_sort(String::from("anagram"), String::from("nagaram"));
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let res = is_anagram_sort(String::from("rat"), String::from("car"));
        let expected = false;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let res = is_anagram_sort(String::from("aa"), String::from("a"));
        let expected = false;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let res = is_anagram_sort(String::from("ggii"), String::from("eekk"));
        let expected = false;
        assert_eq!(res, expected);
    }
}

#[cfg(test)]
mod tests_frequency {
    use super::*;

    #[test]
    fn test_1() {
        let res = is_anagram_frequency(String::from("anagram"), String::from("nagaram"));
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let res = is_anagram_frequency(String::from("rat"), String::from("car"));
        let expected = false;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let res = is_anagram_frequency(String::from("aa"), String::from("a"));
        let expected = false;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let res = is_anagram_frequency(String::from("ggii"), String::from("eekk"));
        let expected = false;
        assert_eq!(res, expected);
    }
}

#[cfg(test)]
mod tests_hashmap {
    use super::*;

    #[test]
    fn test_1() {
        let res = is_anagram_hashmap(String::from("anagram"), String::from("nagaram"));
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let res = is_anagram_hashmap(String::from("rat"), String::from("car"));
        let expected = false;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let res = is_anagram_hashmap(String::from("aa"), String::from("a"));
        let expected = false;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let res = is_anagram_hashmap(String::from("ggii"), String::from("eekk"));
        let expected = false;
        assert_eq!(res, expected);
    }
}
