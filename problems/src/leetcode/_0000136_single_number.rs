// 0000136. Single Number
// https://leetcode.com/problems/single-number/description/
// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
//
// You must implement a solution with a linear runtime complexity and use only constant extra space.
//
// Example 1:
//
// Input: nums = [2,2,1]
// Output: 1
//
// Example 2:
//
// Input: nums = [4,1,2,1,2]
// Output: 4
//
// Example 3:
//
// Input: nums = [1]
// Output: 1
//
// Constraints:
//
//     1 <= nums.length <= 3 * 104
//     -3 * 104 <= nums[i] <= 3 * 104
//     Each element in the array appears twice except for one element which appears only once.
//
//
// ---
pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().reduce(|a, b| a ^ b).unwrap()
}
// ---

pub fn testcase() {
    let nums = vec![2, 2, 1];
    let res = single_number(nums);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 2, 1];
        let res = single_number(nums);
        let expected = 1;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 1, 2, 1, 2];
        let res = single_number(nums);
        let expected = 4;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![1];
        let res = single_number(nums);
        let expected = 1;
        assert_eq!(res, expected);
    }
}
