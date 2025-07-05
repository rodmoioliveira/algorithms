// 0001657. Determine if Two Strings Are Close
// https://leetcode.com/problems/determine-if-two-strings-are-close/description/
// Two strings are considered close if you can attain one from the other using the following operations...
//
// - Operation 1: Swap any two existing characters.
//     For example, abcde -> aecdb
// - Operation 2: Transform every occurrence of one existing character into another existing character, and do the same with the other character.
//     For example, aacabb -> bbcbaa (all a's turn into b's, and all b's turn into a's)
//
// You can use the operations on either string as many times as necessary.
//
// Given two strings, word1 and word2, return true if word1 and word2 are close, and false otherwise.
//
// Example 1:
//
// Input: word1 = "abc", word2 = "bca"
// Output: true
// Explanation: You can attain word2 from word1 in 2 operations.
// Apply Operation 1: "abc" -> "acb"
// Apply Operation 1: "acb" -> "bca"
//
// Example 2:
//
// Input: word1 = "a", word2 = "aa"
// Output: false
// Explanation: It is impossible to attain word2 from word1, or vice versa, in any number of operations.
//
// Example 3:
//
// Input: word1 = "cabbba", word2 = "abbccc"
// Output: true
// Explanation: You can attain word2 from word1 in 3 operations.
// Apply Operation 1: "cabbba" -> "caabbb"
// Apply Operation 2: "caabbb" -> "baaccc"
// Apply Operation 2: "baaccc" -> "abbccc"
//
// Constraints:
//
//     1 <= word1.length, word2.length <= 105
//     word1 and word2 contain only lowercase English letters.
// ---
pub fn close_strings_hashmap(word1: String, word2: String) -> bool {
    use std::collections::{HashMap, HashSet};

    let mut w1: HashMap<char, usize> = HashMap::new();
    let mut w2: HashMap<char, usize> = HashMap::new();

    for c in word1.chars() {
        w1.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    for c in word2.chars() {
        w2.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    let w1_keys: HashSet<&char> = HashSet::from_iter(w1.keys());
    let w2_keys: HashSet<&char> = HashSet::from_iter(w2.keys());

    let mut w1_values: Vec<&usize> = w1.values().collect();
    let mut w2_values: Vec<&usize> = w2.values().collect();
    w1_values.sort();
    w2_values.sort();

    w1_keys == w2_keys && w1_values == w2_values
}

pub fn close_strings_bitwise(word1: String, word2: String) -> bool {
    let mut bw1: u32 = 0;
    let mut bw2: u32 = 0;

    let mut fw1: [usize; 26] = [0; 26];
    let mut fw2: [usize; 26] = [0; 26];

    let a = b'a';

    for c in word1.bytes() {
        let index = c - a;
        fw1[index as usize] += 1;
        bw1 |= 1 << index;
    }

    for c in word2.bytes() {
        let index = c - a;
        fw2[index as usize] += 1;
        bw2 |= 1 << index;
    }

    fw1.sort();
    fw2.sort();

    bw1 == bw2 && fw1 == fw2
}
// ---

pub fn testcase() {
    let res = close_strings_bitwise(
        "aaabbbbccddeeeeefffff".to_owned(),
        "aaaaabbcccdddeeeeffff".to_owned(),
    );
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests_hasmap {
    use super::*;

    #[test]
    fn test_1() {
        let res = close_strings_hashmap("abc".to_owned(), "bca".to_owned());
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let res = close_strings_hashmap("a".to_owned(), "aa".to_owned());
        let expected = false;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let res = close_strings_hashmap("cabbba".to_owned(), "abbccc".to_owned());
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let res = close_strings_hashmap(
            "aaabbbbccddeeeeefffff".to_owned(),
            "aaaaabbcccdddeeeeffff".to_owned(),
        );
        let expected = false;
        assert_eq!(res, expected);
    }
}

#[cfg(test)]
mod tests_bitwise {
    use super::*;

    #[test]
    fn test_1() {
        let res = close_strings_bitwise("abc".to_owned(), "bca".to_owned());
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let res = close_strings_bitwise("a".to_owned(), "aa".to_owned());
        let expected = false;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let res = close_strings_bitwise("cabbba".to_owned(), "abbccc".to_owned());
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let res = close_strings_bitwise(
            "aaabbbbccddeeeeefffff".to_owned(),
            "aaaaabbcccdddeeeeffff".to_owned(),
        );
        let expected = false;
        assert_eq!(res, expected);
    }
}
