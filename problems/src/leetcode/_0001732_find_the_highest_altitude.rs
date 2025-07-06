// 0001732. Find the Highest Altitude
// https://leetcode.com/problems/find-the-highest-altitude/description/
// There is a biker going on a road trip.
//
// The road trip consists of n + 1 points at different altitudes. The biker starts his trip on
// point 0 with altitude equal 0.
//
// You are given an integer array `gain` of length `n` where `gain[i]` is the net gain in altitude
// between points `i` and `i + 1` for all `(0 <= i < n)`. Return the highest altitude of a point.
//
// Example 1:
//
// Input: gain = [-5,1,5,0,-7]
// Output: 1
// Explanation: The altitudes are [0,-5,-4,1,1,-6]. The highest is 1.
//
// Example 2:
//
// Input: gain = [-4,-3,-2,-1,4,3,2]
// Output: 0
// Explanation: The altitudes are [0,-4,-7,-9,-10,-6,-3,-1]. The highest is 0.
//
// Constraints:
//
//     n == gain.length
//     1 <= n <= 100
//     -100 <= gain[i] <= 100
// ---
pub fn largest_altitude_loop(gain: Vec<i32>) -> i32 {
    let mut alt_cur = 0;
    let mut alt_high = alt_cur;

    for g in gain {
        alt_cur += g;
        alt_high = std::cmp::max(alt_cur, alt_high);
    }

    alt_high
}

pub fn largest_altitude_scan(gain: Vec<i32>) -> i32 {
    gain.iter()
        .scan(0, |alt, g| {
            *alt += g;
            Some(*alt)
        })
        .max()
        .expect("expect highest altitude value")
        .max(0)
}
// ---

pub fn testcase() {
    let gain = vec![-5, 1, 5, 0, -7];
    let actual = largest_altitude_scan(gain);
    eprintln!("{} {:?}", module_path!(), actual);
}

#[cfg(test)]
mod tests_scan {
    use super::*;

    #[test]
    fn test_1() {
        let gain = vec![-5, 1, 5, 0, -7];
        let actual = largest_altitude_scan(gain);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let gain = vec![-4, -3, -2, -1, 4, 3, 2];
        let actual = largest_altitude_scan(gain);
        let expected = 0;
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod tests_loop {
    use super::*;

    #[test]
    fn test_1() {
        let gain = vec![-5, 1, 5, 0, -7];
        let actual = largest_altitude_loop(gain);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let gain = vec![-4, -3, -2, -1, 4, 3, 2];
        let actual = largest_altitude_loop(gain);
        let expected = 0;
        assert_eq!(actual, expected);
    }
}
