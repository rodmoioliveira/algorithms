// 0000704. Binary Search
// https://leetcode.com/problems/binary-search/description/
// Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums.
//
// If target exists, then return its index. Otherwise, return -1.
// You must write an algorithm with O(log n) runtime complexity.
//
// Example 1:
//
// Input: nums = [-1,0,3,5,9,12], target = 9
// Output: 4
// Explanation: 9 exists in nums and its index is 4
//
// Example 2:
//
// Input: nums = [-1,0,3,5,9,12], target = 2
// Output: -1
// Explanation: 2 does not exist in nums so return -1
//
// Constraints:
//
//     1 <= nums.length <= 104
//     -104 < nums[i], target < 104
//     All the integers in nums are unique.
//     nums is sorted in ascending order.
// ---
pub fn search_1(nums: Vec<i32>, target: i32) -> i32 {
    let l = nums.len();
    let mut lo = 0;
    let mut mid = l / 2;

    while mid >= 1 {
        while lo + mid < l && nums[lo + mid] <= target {
            lo += mid
        }
        mid /= 2;
    }

    if nums[lo] == target {
        lo as i32
    } else {
        -1
    }
}

pub fn search_2(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo = 0;
    let mut hi = nums.len();

    while lo < hi {
        let mid = (lo + hi) / 2;

        if nums[mid] >= target {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    if nums[lo] == target {
        lo as i32
    } else {
        -1
    }
}

pub fn search_3(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(k) => k as i32,
        Err(_) => -1,
    }
}
// ---

pub fn testcase() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let res = search_1(nums, 9);
    eprintln!("{} {:?}", module_path!(), res);

    let nums = vec![-1, 0, 3, 5, 9, 12];
    let res = search_2(nums, 9);
    eprintln!("{} {:?}", module_path!(), res);

    let nums = vec![-1, 0, 3, 5, 9, 12];
    let res = search_3(nums, 9);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests_1 {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let res = search_1(nums, 9);
        let expected = 4;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let res = search_1(nums, 2);
        let expected = -1;
        assert_eq!(res, expected);
    }
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let res = search_2(nums, 9);
        let expected = 4;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let res = search_2(nums, 2);
        let expected = -1;
        assert_eq!(res, expected);
    }
}

#[cfg(test)]
mod tests_3 {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let res = search_3(nums, 9);
        let expected = 4;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let res = search_3(nums, 2);
        let expected = -1;
        assert_eq!(res, expected);
    }
}
