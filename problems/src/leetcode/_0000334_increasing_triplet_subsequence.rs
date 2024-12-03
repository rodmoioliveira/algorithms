// 0000334. Increasing Triplet Subsequence
// https://leetcode.com/problems/increasing-triplet-subsequence/description
// Given an integer array nums, return true if there exists a triple of indices.
//
// Given an integer array nums, return true if there exists a triple of indices (i, j, k) such that
// i < j < k and nums[i] < nums[j] < nums[k]. If no such indices exists, return false.
//
// Example 1:
//
// Input: nums = [1,2,3,4,5]
// Output: true
// Explanation: Any triplet where i < j < k is valid.
//
// Example 2:
//
// Input: nums = [5,4,3,2,1]
// Output: false
// Explanation: No triplet exists.
//
// Example 3:
//
// Input: nums = [2,1,5,0,4,6]
// Output: true
// Explanation: The triplet (3, 4, 5) is valid because nums[3] == 0 < nums[4] == 4 < nums[5] == 6.
//
// Constraints:
//
//     1 <= nums.length <= 5 * 105
//     -231 <= nums[i] <= 231 - 1
//
// Follow up: Could you implement a solution that runs in O(n) time complexity and O(1) space complexity?
//
// ---
pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut bottom = i32::MAX;
    let mut middle = i32::MAX;

    for n in nums {
        if n <= bottom {
            bottom = n;
        } else if n <= middle {
            middle = n;
        } else if n > middle {
            return true;
        }
    }
    false
}
// ---

pub fn testcase() {
    let nums = vec![1, 2, 3, 4, 5];
    let res = increasing_triplet(nums);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4, 5];
        let res = increasing_triplet(nums);
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![5, 4, 3, 2, 1];
        let res = increasing_triplet(nums);
        let expected = false;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![2, 1, 5, 0, 4, 6];
        let res = increasing_triplet(nums);
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let nums = vec![1, 1, -2, 6];
        let res = increasing_triplet(nums);
        let expected = false;
        assert_eq!(res, expected);
    }
}
