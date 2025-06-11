// 0000026. Remove Duplicates from Sorted Array
// https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/
// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once.
//
// The relative order of the elements should be kept
// the same. Then return the number of unique elements in nums.
//
// Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:
//
//     Change the array nums such that the first k elements of nums contain the unique elements in
//     the order they were present in nums initially. The remaining elements of nums are not
//     important as well as the size of nums.
//
//     Return k.
//
// Custom Judge:
//
// The judge will test your solution with the following code:
//
// int[] nums = [...]; // Input array
// int[] expectedNums = [...]; // The expected answer with correct length
//
// int k = removeDuplicates(nums); // Calls your implementation
//
// assert k == expectedNums.length;
// for (int i = 0; i < k; i++) {
//     assert nums[i] == expectedNums[i];
// }
//
// If all assertions pass, then your solution will be accepted.
//
// Example 1:
//
// Input: nums = [1,1,2]
// Output: 2, nums = [1,2,_]
// Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2 respectively.
// It does not matter what you leave beyond the returned k (hence they are underscores).
//
// Example 2:
//
// Input: nums = [0,0,1,1,1,2,2,3,3,4]
// Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
// Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4 respectively.
// It does not matter what you leave beyond the returned k (hence they are underscores).
//
// Constraints:
//
//     1 <= nums.length <= 3 * 104
//     -100 <= nums[i] <= 100
//     nums is sorted in non-decreasing order.
// ---
// Sets approach
pub fn remove_duplicates_1(nums: &mut Vec<i32>) -> i32 {
    use std::collections::BTreeSet;

    let s: BTreeSet<i32> = BTreeSet::from_iter(nums.clone());
    let l = s.len() as i32;
    *nums = s.into_iter().collect();
    l
}

// Two pointers approach
#[allow(clippy::ptr_arg)]
pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
    let mut unique_idx = 1_usize;

    for dup_idx in 1..nums.len() {
        if nums[dup_idx] != nums[dup_idx - 1] {
            nums[unique_idx] = nums[dup_idx];
            unique_idx += 1;
        }
    }
    unique_idx as i32
}
// ---

pub fn testcase() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let res = remove_duplicates_1(&mut nums);
    eprintln!("{} {:?} {:?}", module_path!(), res, nums);

    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let res = remove_duplicates_2(&mut nums);
    eprintln!("{} {:?} {:?}", module_path!(), res, nums);
}

#[cfg(test)]
mod tests_1 {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let res = remove_duplicates_1(&mut nums);
        let expected_res = 5;
        assert_eq!(res, expected_res);

        let res_num = &nums[0..(res as usize)];
        let expected_nums = vec![0, 1, 2, 3, 4];
        assert_eq!(res_num, expected_nums);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![1, 1, 2];
        let res = remove_duplicates_1(&mut nums);
        let expected_res = 2;
        assert_eq!(res, expected_res);

        let res_num = &nums[0..(res as usize)];
        let expected_nums = vec![1, 2];
        assert_eq!(res_num, expected_nums);
    }
}


#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let res = remove_duplicates_2(&mut nums);
        let expected_res = 5;
        assert_eq!(res, expected_res);

        let expected_nums = vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4];
        assert_eq!(nums, expected_nums);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![1, 1, 2];
        let res = remove_duplicates_2(&mut nums);
        let expected_res = 2;
        assert_eq!(res, expected_res);

        let expected_nums = vec![1, 2, 2];
        assert_eq!(nums, expected_nums);
    }
}
