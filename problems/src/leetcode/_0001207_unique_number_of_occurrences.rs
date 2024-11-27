// 0001207. Unique Number of Occurrences
// https://leetcode.com/problems/unique-number-of-occurrences/description/
// Return true if the number of occurrences of each value in the array is unique or false otherwise.
//
// Given an array of integers arr, return true if the number of occurrences of each value in the array is unique or false otherwise.
//
// Example 1:
//
// Input: arr = [1,2,2,1,1,3]
// Output: true
// Explanation: The value 1 has 3 occurrences, 2 has 2 and 3 has 1. No two values have the same number of occurrences.
//
// Example 2:
//
// Input: arr = [1,2]
// Output: false
//
// Example 3:
//
// Input: arr = [-3,0,1,-3,1,1,1,-3,10,0]
// Output: true
//
// Constraints:
//
//     1 <= arr.length <= 1000
//     -1000 <= arr[i] <= 1000
//
// ---
pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    use std::collections::{HashMap, HashSet};

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(arr.len());
    for i in arr {
        map.entry(i).and_modify(|c| *c += 1).or_insert(1);
    }
    let set: HashSet<&i32> = HashSet::from_iter(map.values());
    map.values().len() == set.len()
}
// ---

pub fn testcase() {
    let arr = vec![1, 2, 2, 1, 1, 3];
    let res = unique_occurrences(arr);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![1, 2, 2, 1, 1, 3];
        let res = unique_occurrences(arr);
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let arr = vec![1, 2];
        let res = unique_occurrences(arr);
        let expected = false;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        let res = unique_occurrences(arr);
        let expected = true;
        assert_eq!(res, expected);
    }
}
