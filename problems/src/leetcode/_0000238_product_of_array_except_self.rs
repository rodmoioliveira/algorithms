// 0000238. Product of Array Except Self
// https://leetcode.com/problems/product-of-array-except-self/description/
// Return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
//
// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
//
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
//
// You must write an algorithm that runs in O(n) time and without using the division operation.
//
// Example 1:
//
// Input: nums = [1,2,3,4]
// Output: [24,12,8,6]
//
// Example 2:
//
// Input: nums = [-1,1,0,-3,3]
// Output: [0,0,9,0,0]
//
// Constraints:
//
//     2 <= nums.length <= 105
//     -30 <= nums[i] <= 30
//     The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
//
// Follow up: Can you solve the problem in O(1) extra space complexity? (The output array does not count as extra space for space complexity analysis.)
//
// ---
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let nums_len = nums.len();
    let last_idx = nums_len - 1;
    let mut prefix: Vec<i32> = vec![1; nums_len];
    let mut suffix: Vec<i32> = vec![1; nums_len];
    let mut result: Vec<i32> = vec![1; nums_len];

    //   INPUT:       1   2   3   4
    // ----------------------------
    //  PREFIX:       1   1   2   6
    // PRODUCT:       *   *   *   *
    //  SUFFIX:      24  12   4   1
    // ----------------------------
    //  OUTPUT:      24  12   8   6

    for i in 0..nums_len {
        if i != last_idx {
            prefix[i + 1] = nums[i] * prefix[i];
        }

        let j = last_idx - i;
        if j != last_idx {
            suffix[j] = nums[j + 1] * suffix[j + 1];
        }
    }

    for i in 0..nums_len {
        result[i] = prefix[i] * suffix[i];
    }

    result
}
// ---

pub fn testcase() {
    let nums = vec![1, 2, 3, 4];
    let result = product_except_self(nums);
    eprintln!(
        "leetcode/0000238_product_of_array_except_self: {:?}",
        result
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4];
        let result = product_except_self(nums);
        let expected = vec![24, 12, 8, 6];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let result = product_except_self(nums);
        let expected = vec![0, 0, 9, 0, 0];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![-1, 1, 0, -3, 0];
        let result = product_except_self(nums);
        let expected = vec![0, 0, 0, 0, 0];
        assert_eq!(result, expected);
    }
}
