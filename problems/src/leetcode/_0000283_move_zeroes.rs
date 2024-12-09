// 0000283. Move Zeroes
// https://leetcode.com/problems/move-zeroes/description/
// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
//
// Note that you must do this in-place without making a copy of the array.
//
// Example 1:
//
// Input: nums = [0,1,0,3,12]
// Output: [1,3,12,0,0]
//
// Example 2:
//
// Input: nums = [0]
// Output: [0]
//
// Constraints:
//
//     1 <= nums.length <= 104
//     -231 <= nums[i] <= 231 - 1
//
// Follow up: Could you minimize the total number of operations done?
// ---
pub fn move_zeroes(nums: &mut [i32]) {
    let mut j = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(i, j);
            j += 1;
        }
    }
}
// ---

pub fn testcase() {
    let mut nums = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut nums);
    eprintln!("{} {:?}", module_path!(), nums);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        let expected = vec![1, 3, 12, 0, 0];
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0];
        move_zeroes(&mut nums);
        let expected = vec![0];
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_3() {
        let mut nums = vec![0, 0, 1, 3, 12];
        move_zeroes(&mut nums);
        let expected = vec![1, 3, 12, 0, 0];
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_4() {
        let mut nums = vec![1, 3, 12, 0, 0];
        move_zeroes(&mut nums);
        let expected = vec![1, 3, 12, 0, 0];
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_5() {
        let mut nums = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        move_zeroes(&mut nums);
        let expected = vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(nums, expected);
    }
}
