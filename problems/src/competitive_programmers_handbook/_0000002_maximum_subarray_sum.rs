// =============
// Maximum subarray sum
// ---------
// Laaksonen, Antti. Competitive Programmer's Handbook, p.21.
// =============
//
// There are often several possible algorithms for solving a problem such that their time
// complexities are different. This section discusses a classic problem that has a straightforward
// O(n^3) solution. However, by designing a better algorithm, it is possible to solve the problem
// in O(n^2) time and even in O(n) time.
//
// Given an array of n numbers, our task is to calculate the maximum subarray sum, i.e., the
// largest possible sum of a sequence of consecutive values in the array. The problem is
// interesting when there may be negative values in the array. For example, in the array
//
// [−1 2 4 −3 5 2 −5 2]
//
// the following subarray produces the maximum sum 10:
//
//     |--------|
// [−1 2 4 −3 5 2 −5 2]
//
// We assume that an empty subarray is allowed, so the maximum subarray sum is always at least 0.
// ---
// =============
// Algorithm 1
// =============
//
// O(n^3) time complexity
//
// A straightforward way to solve the problem is to go through all possible subarrays, calculate
// the sum of values in each subarray and maintain the maximum sum.
//
// The variables a and b fix the first and last index of the subarray, and the sum of values is
// calculated to the variable sum. The variable best contains the maximum sum found during the
// search.
//
// The time complexity of the algorithm is O (n^3), because it consists of three nested loops that
// go through the input.
pub fn maximum_subarray_sum_on3(nums: &[i32]) -> i32 {
    let mut best = 0;
    let nums_len = nums.len();

    for a in 0..nums_len {
        for b in a..nums_len {
            let mut sum = 0;

            #[allow(clippy::needless_range_loop)]
            for k in a..=b {
                sum += nums[k];
            }
            best = std::cmp::max(best, sum);
        }
    }
    best
}

// =============
// Algorithm 2
// =============
//
// O(n^2) time complexity
//
// It is easy to make Algorithm 1 more efficient by removing one loop from it. This is possible by
// calculating the sum at the same time when the right end of the subarray moves.
pub fn maximum_subarray_sum_on2(nums: &[i32]) -> i32 {
    let mut best = 0;
    let nums_len = nums.len();

    for a in 0..nums_len {
        let mut sum = 0;

        #[allow(clippy::needless_range_loop)]
        for b in a..nums_len {
            sum += nums[b];
            best = std::cmp::max(best, sum);
        }
    }
    best
}

// =============
// Algorithm 3
// =============
//
// O(n) time complexity
//
// Surprisingly, it is possible to solve the problem in O(n) time , which means that just one loop
// is enough. The idea is to calculate, for each array position, the maximum sum of a subarray that
// ends at that position. After this, the answer for the problem is the maximum of those sums.
//
// Consider the subproblem of finding the maximum-sum subarray that ends at position k. There are
// two possibilities:
//
// 1. The subarray only contains the element at position k.
// 2. The subarray consists of a subarray that ends at position k − 1, followed by the element at
//    position k.
//
// In the latter case, since we want to find a subarray with maximum sum, the subarray that ends at
// position k − 1 should also have the maximum sum. Thus, we can solve the problem efficiently by
// calculating the maximum subarray sum for each ending position from left to right.
pub fn maximum_subarray_sum_on1(nums: &[i32]) -> i32 {
    let mut best = 0;
    let mut sum = 0;

    #[allow(clippy::needless_range_loop)]
    for k in 0..nums.len() {
        sum = std::cmp::max(nums[k], sum + nums[k]);
        best = std::cmp::max(best, sum);
    }
    best
}

pub fn maximum_subarray_sum_on1_no_needless_range_loop(nums: &[i32]) -> i32 {
    let mut best = 0;
    let mut sum = 0;

    for k in nums {
        sum = std::cmp::max(*k, sum + *k);
        best = std::cmp::max(best, sum);
    }
    best
}
// ---

pub fn testcase() {
    let nums = vec![-1, 2, 4, -3, 5, 2, -5, 2];

    let res = maximum_subarray_sum_on3(&nums);
    eprintln!("{} {:?}", module_path!(), res);
    let res = maximum_subarray_sum_on2(&nums);
    eprintln!("{} {:?}", module_path!(), res);
    let res = maximum_subarray_sum_on1(&nums);
    eprintln!("{} {:?}", module_path!(), res);
    let res = maximum_subarray_sum_on1_no_needless_range_loop(&nums);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests_on3 {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![-1, 2, 4, -3, 5, 2, -5, 2];
        let res = maximum_subarray_sum_on3(&nums);
        let expected = 10;
        assert_eq!(res, expected);
    }
}

#[cfg(test)]
mod tests_on2 {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![-1, 2, 4, -3, 5, 2, -5, 2];
        let res = maximum_subarray_sum_on2(&nums);
        let expected = 10;
        assert_eq!(res, expected);
    }
}

#[cfg(test)]
mod tests_on1 {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![-1, 2, 4, -3, 5, 2, -5, 2];
        let res = maximum_subarray_sum_on1(&nums);
        let expected = 10;
        assert_eq!(res, expected);
    }
}

#[cfg(test)]
mod tests_on1_no_needless_range_loop {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![-1, 2, 4, -3, 5, 2, -5, 2];
        let res = maximum_subarray_sum_on1_no_needless_range_loop(&nums);
        let expected = 10;
        assert_eq!(res, expected);
    }
}
