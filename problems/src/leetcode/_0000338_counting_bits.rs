// 0000338. Counting Bits
// https://leetcode.com/problems/counting-bits/description/
// Given an integer n, return an array ans of length n + 1 with the binary representation of i.
//
// Given an integer n, return an array ans of length n + 1 such that for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.
//
// Example 1:
//
// Input: n = 2
// Output: [0,1,1]
// Explanation:
// 0 --> 0
// 1 --> 1
// 2 --> 10
//
// Example 2:
//
// Input: n = 5
// Output: [0,1,1,2,1,2]
// Explanation:
// 0 --> 0
// 1 --> 1
// 2 --> 10
// 3 --> 11
// 4 --> 100
// 5 --> 101
//
// Constraints:
//
//     0 <= n <= 105
//
// Follow up:
//
// - It is very easy to come up with a solution with a runtime of O(n log n). Can you do it in linear time O(n) and possibly in a single pass?
// - Can you do it without using any built-in function (i.e., like __builtin_popcount in C++)?
//
// ---
pub fn count_bits(n: i32) -> Vec<i32> {
    Vec::from_iter(0..=n)
        .into_iter()
        .map(|n| n.count_ones() as i32)
        .collect()
}

pub fn count_bits_2(n: i32) -> Vec<i32> {
    let mut ans = vec![0; (n + 1) as usize];
    for i in 1..=n {
        let a = i & (i - 1);
        ans[i as usize] = ans[a as usize] + 1;
    }
    ans
}
// ---

pub fn testcase() {
    let n = 20;
    let res = count_bits(n);
    eprintln!("{} {:?}", module_path!(), res);

    let n = 20;
    let res = count_bits_2(n);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 2;
        let expected = vec![0, 1, 1];

        let res = count_bits(n);
        assert_eq!(res, expected);

        let res = count_bits_2(n);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let n = 5;
        let expected = vec![0, 1, 1, 2, 1, 2];

        let res = count_bits(n);
        assert_eq!(res, expected);

        let res = count_bits_2(n);
        assert_eq!(res, expected);
    }
}
