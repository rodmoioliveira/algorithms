// 0000053. Maximum Subarray
// https://leetcode.com/problems/maximum-subarray/description/
// Given an integer array nums, find the with the largest sum, and return its sum.
//
// Example 1:
//
// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
// Output: 6
// Explanation: The subarray [4,-1,2,1] has the largest sum 6.
//
// Example 2:
//
// Input: nums = [1]
// Output: 1
// Explanation: The subarray [1] has the largest sum 1.
//
// Example 3:
//
// Input: nums = [5,4,-1,7,8]
// Output: 23
// Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
//
// Constraints:
//
//     1 <= nums.length <= 105
//     -104 <= nums[i] <= 104
//
// Follow up: If you have figured out the O(n) solution, try coding another solution using the
// divide and conquer approach, which is more subtle.
// ---
// https://en.m.wikipedia.org/wiki/Maximum_subarray_problem#Kadane's_algorithm
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut best = i32::MIN;
    let mut sum = 0;

    for k in nums {
        sum = std::cmp::max(k, sum + k);
        best = std::cmp::max(best, sum);
    }
    best
}
// ---

pub fn testcase() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let res = max_sub_array(nums);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests_1 {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let result = max_sub_array(nums);
        let expected = 6;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![1];
        let result = max_sub_array(nums);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![5, 4, -1, 7, 8];
        let result = max_sub_array(nums);
        let expected = 23;
        assert_eq!(result, expected);
    }
}
