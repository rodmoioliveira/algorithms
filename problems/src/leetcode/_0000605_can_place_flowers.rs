// 0000605. Can Place Flowers
// https://leetcode.com/problems/can-place-flowers/description/
// You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.
//
// Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return true if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule and false otherwise.
//
// Example 1:
//
// Input: flowerbed = [1,0,0,0,1], n = 1
// Output: true
//
// Example 2:
//
// Input: flowerbed = [1,0,0,0,1], n = 2
// Output: false
//
// Constraints:
//
//     1 <= flowerbed.length <= 2 * 104
//     flowerbed[i] is 0 or 1.
//     There are no two adjacent flowers in flowerbed.
//     0 <= n <= flowerbed.length
//
// ---
pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut flowerbed_mut = flowerbed.clone();
    let flowerbed_len = flowerbed_mut.len();
    let mut slots_available = 0;

    for i in 0..flowerbed_len {
        let left = i == 0 || flowerbed_mut[i - 1] == 0;
        let right = i == flowerbed_len - 1 || flowerbed_mut[i + 1] == 0;
        if flowerbed_mut[i] == 0 && left && right {
            flowerbed_mut[i] = 1;
            slots_available += 1;
        }
    }
    slots_available >= n
}
// ---

pub fn testcase() {
    let flowerbed = vec![1, 0, 0, 0, 1];
    let n = 1;
    let result = can_place_flowers(flowerbed, n);
    eprintln!("0000605_can_place_flowers: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        let result = can_place_flowers(flowerbed, n);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        let result = can_place_flowers(flowerbed, n);
        let expected = false;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let flowerbed = vec![1, 0, 0, 0, 0, 1];
        let n = 2;
        let result = can_place_flowers(flowerbed, n);
        let expected = false;
        assert_eq!(result, expected);
    }
}
