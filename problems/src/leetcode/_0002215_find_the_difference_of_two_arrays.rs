// 0002215. Find the Difference of Two Arrays
// https://leetcode.com/problems/find-the-difference-of-two-arrays/description/
// Given two 0-indexed integer arrays nums1 and nums2, return a list answer of size 2.
//
// Where:
//
// - answer[0] is a list of all distinct integers in nums1 which are not present in nums2.
// - answer[1] is a list of all distinct integers in nums2 which are not present in nums1.
//
// Note that the integers in the lists may be returned in any order.
//
// Example 1:
//
// Input: nums1 = [1,2,3], nums2 = [2,4,6]
// Output: [[1,3],[4,6]]
// Explanation:
// For nums1, nums1[1] = 2 is present at index 0 of nums2, whereas nums1[0] = 1 and nums1[2] = 3 are not present in nums2. Therefore, answer[0] = [1,3].
// For nums2, nums2[0] = 2 is present at index 1 of nums1, whereas nums2[1] = 4 and nums2[2] = 6 are not present in nums2. Therefore, answer[1] = [4,6].
//
// Example 2:
//
// Input: nums1 = [1,2,3,3], nums2 = [1,1,2,2]
// Output: [[3],[]]
// Explanation:
// For nums1, nums1[2] and nums1[3] are not present in nums2. Since nums1[2] == nums1[3], their value is only included once and answer[0] = [3].
// Every integer in nums2 is present in nums1. Therefore, answer[1] = [].
//
// Constraints:
//
//     1 <= nums1.length, nums2.length <= 1000
//     -1000 <= nums1[i], nums2[i] <= 1000
//
// ---
pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::hash_set::HashSet;

    let set1: HashSet<i32> = HashSet::from_iter(nums1);
    let set2: HashSet<i32> = HashSet::from_iter(nums2);
    let diff1: Vec<i32> = set1.difference(&set2).cloned().collect();
    let diff2: Vec<i32> = set2.difference(&set1).cloned().collect();
    vec![diff1, diff2]
}

// ---

pub fn testcase() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![2, 4, 6];
    let result = find_difference(nums1, nums2);

    eprintln!(
        "leetcode/0002215_find_the_difference_of_two_arrays: {:?}",
        result
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_set::HashSet;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        let result = find_difference(nums1, nums2)
            .into_iter()
            .map(HashSet::from_iter)
            .collect::<Vec<HashSet<i32>>>();
        let expected = vec![[1, 3], [4, 6]]
            .into_iter()
            .map(HashSet::from_iter)
            .collect::<Vec<HashSet<i32>>>();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        let result = find_difference(nums1, nums2)
            .into_iter()
            .map(HashSet::from_iter)
            .collect::<Vec<HashSet<i32>>>();
        let expected = vec![vec![3], vec![]]
            .into_iter()
            .map(HashSet::from_iter)
            .collect::<Vec<HashSet<i32>>>();
        assert_eq!(result, expected);
    }
}
