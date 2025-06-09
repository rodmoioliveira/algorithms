// 0000198. House Robber
// https://leetcode.com/problems/house-robber/description/
// You are a professional robber planning to rob houses along a street.
//
// Each house has a certain amount of money stashed, the only constraint stopping you from robbing
// each of them is that adjacent houses have security systems connected and it will automatically
// contact the police if two adjacent houses were broken into on the same night.
//
// Given an integer array nums representing the amount of money of each house, return the maximum
// amount of money you can rob tonight without alerting the police.
//
// Example 1:
//
// Input: nums = [1,2,3,1]
// Output: 4
// Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
// Total amount you can rob = 1 + 3 = 4.
//
// Example 2:
//
// Input: nums = [2,7,9,3,1]
// Output: 12
// Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
// Total amount you can rob = 2 + 9 + 1 = 12.
//
// Constraints:
//
//     1 <= nums.length <= 100
//     0 <= nums[i] <= 400
//
// ---
pub fn rob(nums: Vec<i32>) -> i32 {
    // dp[i] => when robber reaches i, max profit he can achieve
    // dp[i] = max {
    //     nums[i] + dp[i - 2] = rob_last_but_one
    //               dp[i - 1] = rob_last
    // }
    let mut rob_last_but_one = 0;
    let mut rob_last = 0;
    for num in nums {
        let rob_max = std::cmp::max(num + rob_last_but_one, rob_last);
        rob_last_but_one = rob_last;
        rob_last = rob_max;
    }
    rob_last
}
// ---

pub fn testcase() {
    let nums = vec![2, 7, 9, 3, 1];
    let res = rob(nums);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 1];
        let res = rob(nums);
        let expected = 4;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 7, 9, 3, 1];
        let res = rob(nums);
        let expected = 12;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![0];
        let res = rob(nums);
        let expected = 0;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let nums = vec![10, 1, 0, 1, 10];
        let res = rob(nums);
        let expected = 20;
        assert_eq!(res, expected);
    }
}
