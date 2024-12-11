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
//
// ======================
// Previous Approaches
// ======================
pub fn _move_zeroes_previous_approach_1(nums: &mut [i32]) {
    let mut j = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(i, j);
            j += 1;
        }
    }
}
// ---
pub fn move_zeroes(nums: &mut [i32]) {
    let mut zero = 0;
    let mut not_zero = 0;
    let len = nums.len();

    while zero < len && not_zero < len {
        if zero < not_zero {
            nums.swap(not_zero, zero);
        } else {
            not_zero += 1;
        }

        while zero < len && nums[zero] != 0 {
            zero += 1;
        }
        while not_zero < len && nums[not_zero] == 0 {
            not_zero += 1;
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
        let mut nums = vec![1, 3, 12, 0, 1, 0];
        move_zeroes(&mut nums);
        let expected = vec![1, 3, 12, 1, 0, 0];
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_6() {
        let mut nums = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        move_zeroes(&mut nums);
        let expected = vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_7() {
        let mut nums = vec![1, 2, 3, 4, 5];
        move_zeroes(&mut nums);
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(nums, expected);
    }
}
