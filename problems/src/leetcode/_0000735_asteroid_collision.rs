// 0000735. Asteroid Collision
// https://leetcode.com/problems/asteroid-collision/description/
// Find out the state of the asteroids after all collisions.
//
// We are given an array asteroids of integers representing asteroids in a row. The indices of the asteroid in the array represent their relative position in space.
//
// For each asteroid, the absolute value represents its size, and the sign represents its direction
// (positive meaning right, negative meaning left). Each asteroid moves at the same speed.
//
// Find out the state of the asteroids after all collisions. If two asteroids meet, the smaller one
// will explode. If both are the same size, both will explode. Two asteroids moving in the same
// direction will never meet.
//
// Example 1:
//
// Input: asteroids = [5,10,-5]
// Output: [5,10]
// Explanation: The 10 and -5 collide resulting in 10. The 5 and 10 never collide.
//
// Example 2:
//
// Input: asteroids = [8,-8]
// Output: []
// Explanation: The 8 and -8 collide exploding each other.
//
// Example 3:
//
// Input: asteroids = [10,2,-5]
// Output: [10]
// Explanation: The 2 and -5 collide resulting in -5. The 10 and -5 collide resulting in 10.
//
// Constraints:
//
//     2 <= asteroids.length <= 104
//     -1000 <= asteroids[i] <= 1000
//     asteroids[i] != 0
//
// ---
fn is_collision(curr: i32, next: i32) -> bool {
    curr.is_positive() && next.is_negative()
}

pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::with_capacity(asteroids.len());
    let mut pointer = 0;
    stack.push(asteroids[0]);

    while pointer < asteroids.len() - 1 {
        pointer += 1;
        let next = asteroids[pointer];

        if !stack.is_empty() && is_collision(stack[stack.len() - 1], next) {
            let res = next + stack[stack.len() - 1];
            if res == 0 {
                stack.pop();
            }
            if res < 0 {
                stack.pop();
                pointer -= 1;
            }
        } else {
            stack.push(next);
        }
    }

    stack
}
// ---

pub fn testcase() {
    let nums = vec![10, 2, -5];
    let res = asteroid_collision(nums);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![5, 10, -5];
        let res = asteroid_collision(nums);
        let expected = vec![5, 10];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![8, -8];
        let res = asteroid_collision(nums);
        let expected = vec![];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![10, 2, -5];
        let res = asteroid_collision(nums);
        let expected = vec![10];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let nums = vec![-2, -2, 1, -1];
        let res = asteroid_collision(nums);
        let expected = vec![-2, -2];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_5() {
        let nums = vec![-2, 2, -1, -2];
        let res = asteroid_collision(nums);
        let expected = vec![-2];
        assert_eq!(res, expected);
    }
}
