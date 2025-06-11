// 0000035. Search Insert Position
// https://leetcode.com/problems/search-insert-position/description/
// Given a sorted array of distinct integers and a target value, return the index if the target is found.
//
// If not, return the index where it would be if it were inserted in order.
//
// You must write an algorithm with O(log n) runtime complexity.
//
// Example 1:
//
// Input: nums = [1,3,5,6], target = 5
// Output: 2
//
// Example 2:
//
// Input: nums = [1,3,5,6], target = 2
// Output: 1
//
// Example 3:
//
// Input: nums = [1,3,5,6], target = 7
// Output: 4
//
// Constraints:
//
//     1 <= nums.length <= 10^4
//     -10^4 <= nums[i] <= 10^4
//     nums contains distinct values sorted in ascending order.
//     -10^4 <= target <= 10^4
// ---
pub fn search_insert_1(nums: Vec<i32>, target: i32) -> i32 {
    let l = nums.len();
    let mut lo = 0;
    let mut hi = l - 1;

    while lo <= hi {
        let mid = (lo + hi) / 2;

        if nums[mid] == target {
            return mid as i32;
        }

        if nums[mid] < target {
            lo = mid + 1
        }

        if nums[mid] > target && mid == 0 {
            break;
        }

        if nums[mid] > target {
            hi = mid - 1
        }
    }
    lo as i32
}

pub fn search_insert_2(nums: Vec<i32>, target: i32) -> i32 {
    let l = nums.len();
    let mut lo = 0;
    let mut mid = l / 2;

    while mid >= 1 {
        while lo + mid < l && nums[lo + mid] <= target {
            lo += mid
        }
        mid /= 2;
    }

    if nums[lo] >= target {
        lo as i32
    } else {
        (lo + 1) as i32
    }
}

pub fn search_insert_3(nums: Vec<i32>, target: i32) -> i32 {
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
    lo as i32
}

pub fn search_insert_4(nums: Vec<i32>, target: i32) -> i32 {
    nums.binary_search(&target).unwrap_or_else(|x| x) as i32
}

pub fn search_insert_5(nums: Vec<i32>, target: i32) -> i32 {
    nums.partition_point(|&num| num < target) as i32
}
// ---

pub fn testcase() {
    let nums = vec![-1, 0, 1, 2, 3, 4, 5, 6, 8, 9, 10];
    let res = search_insert_1(nums, 7);
    eprintln!("{} {:?}", module_path!(), res);

    let nums = vec![-1, 0, 1, 2, 3, 4, 5, 6, 8, 9, 10];
    let res = search_insert_2(nums, 7);
    eprintln!("{} {:?}", module_path!(), res);

    let nums = vec![-1, 0, 1, 2, 3, 4, 5, 6, 8, 9, 10];
    let res = search_insert_3(nums, 7);
    eprintln!("{} {:?}", module_path!(), res);

    let nums = vec![-1, 0, 1, 2, 3, 4, 5, 6, 8, 9, 10];
    let res = search_insert_4(nums, 7);
    eprintln!("{} {:?}", module_path!(), res);

    let nums = vec![-1, 0, 1, 2, 3, 4, 5, 6, 8, 9, 10];
    let res = search_insert_5(nums, 7);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests_1 {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_1(nums, 5);
        let expected = 2;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_1(nums, 2);
        let expected = 1;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_1(nums, 7);
        let expected = 4;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let nums = vec![-1, 0, 1, 2, 3, 4, 5, 6, 8, 9, 10];
        let res = search_insert_1(nums, 7);
        let expected = 8;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_5() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_1(nums, 0);
        let expected = 0;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_6() {
        let nums = vec![1, 3];
        let res = search_insert_1(nums, 4);
        let expected = 2;
        assert_eq!(res, expected);
    }
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_2(nums, 5);
        let expected = 2;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_2(nums, 2);
        let expected = 1;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_2(nums, 7);
        let expected = 4;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let nums = vec![-1, 0, 1, 2, 3, 4, 5, 6, 8, 9, 10];
        let res = search_insert_2(nums, 7);
        let expected = 8;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_5() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_2(nums, 0);
        let expected = 0;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_6() {
        let nums = vec![1, 3];
        let res = search_insert_2(nums, 4);
        let expected = 2;
        assert_eq!(res, expected);
    }
}

#[cfg(test)]
mod tests_3 {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_3(nums, 5);
        let expected = 2;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_3(nums, 2);
        let expected = 1;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_3(nums, 7);
        let expected = 4;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let nums = vec![-1, 0, 1, 2, 3, 4, 5, 6, 8, 9, 10];
        let res = search_insert_3(nums, 7);
        let expected = 8;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_5() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_3(nums, 0);
        let expected = 0;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_6() {
        let nums = vec![1, 3];
        let res = search_insert_3(nums, 4);
        let expected = 2;
        assert_eq!(res, expected);
    }
}

#[cfg(test)]
mod tests_4 {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_4(nums, 5);
        let expected = 2;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_4(nums, 2);
        let expected = 1;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_4(nums, 7);
        let expected = 4;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let nums = vec![-1, 0, 1, 2, 3, 4, 5, 6, 8, 9, 10];
        let res = search_insert_4(nums, 7);
        let expected = 8;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_5() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_4(nums, 0);
        let expected = 0;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_6() {
        let nums = vec![1, 3];
        let res = search_insert_4(nums, 4);
        let expected = 2;
        assert_eq!(res, expected);
    }
}

#[cfg(test)]
mod tests_5 {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_5(nums, 5);
        let expected = 2;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_5(nums, 2);
        let expected = 1;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_5(nums, 7);
        let expected = 4;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let nums = vec![-1, 0, 1, 2, 3, 4, 5, 6, 8, 9, 10];
        let res = search_insert_5(nums, 7);
        let expected = 8;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_5() {
        let nums = vec![1, 3, 5, 6];
        let res = search_insert_5(nums, 0);
        let expected = 0;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_6() {
        let nums = vec![1, 3];
        let res = search_insert_5(nums, 4);
        let expected = 2;
        assert_eq!(res, expected);
    }
}
