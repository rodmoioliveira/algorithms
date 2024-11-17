// 0000724. Find Pivot Index
// https://leetcode.com/problems/find-pivot-index/description/
// Given an array of integers nums, calculate the pivot index of this array.
//
// The pivot index is the index where the sum of all the numbers strictly to the left of the index is equal to the sum of all the numbers strictly to the index's right.
//
// If the index is on the left edge of the array, then the left sum is 0 because there are no elements to the left. This also applies to the right edge of the array.
//
// Return the leftmost pivot index. If no such index exists, return -1.
//
// Example 1:
//
// Input: nums = [1,7,3,6,5,6]
// Output: 3
// Explanation:
// The pivot index is 3.
// Left sum = nums[0] + nums[1] + nums[2] = 1 + 7 + 3 = 11
// Right sum = nums[4] + nums[5] = 5 + 6 = 11
//
// Example 2:
//
// Input: nums = [1,2,3]
// Output: -1
// Explanation:
// There is no index that satisfies the conditions in the problem statement.
//
// Example 3:
//
// Input: nums = [2,1,-1]
// Output: 0
// Explanation:
// The pivot index is 0.
// Left sum = 0 (no elements to the left of index 0)
// Right sum = nums[1] + nums[2] = 1 + -1 = 0
//
// Constraints:
//
//     1 <= nums.length <= 104
//     -1000 <= nums[i] <= 1000
//
// ---
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let sum_total: i32 = nums.iter().sum();
    let mut sum_left = 0;

    for (pivot, n) in nums.iter().enumerate() {
        let sum_right = sum_total - sum_left - n;
        if sum_left == sum_right {
            return pivot as i32;
        }
        sum_left += n;
    }
    -1
}
// ---

pub fn testcase() {
    let nums = vec![1, 7, 3, 6, 5, 6];
    let result = pivot_index(nums);
    eprintln!("leetcode/0000724_find_pivot_index: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        let result = pivot_index(nums);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3];
        let result = pivot_index(nums);
        let expected = -1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![2, 1, -1];
        let result = pivot_index(nums);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_4() {
        let nums = vec![-1, -1, -1, -1, -1, -1];
        let result = pivot_index(nums);
        let expected = -1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_5() {
        let nums = vec![-1, -1, -1, -1, -1, -1, -1];
        let result = pivot_index(nums);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_6() {
        let nums = vec![-3, -1, -1, -1, -1];
        let result = pivot_index(nums);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_7() {
        let nums = vec![-1, -1, 0, 1, 1, 0];
        let result = pivot_index(nums);
        let expected = 5;
        assert_eq!(result, expected);
    }
}