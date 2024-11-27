// REFERENCES:
// - Dasdan, Ali. Twelve Simple Algorithms to Compute Fibonacci Numbers. https://arxiv.org/pdf/1803.07199
// - Luo, Long. 9 Fibonacci Algorithms: The Most Complete Solutions. https://www.longluo.me/blog/2022/01/29/fibonacci-sequence/#solution-9-math-formula
// - Nayuki. Fast Fibonacci algorithms. https://www.nayuki.io/page/fast-fibonacci-algorithms
// - [ADM] Skiena, Steven S. The Algorithm Design Manual. Third Edition, 2020, ISBN 978-3-030-54255-9.
//
// [ADM, 312]
// We can calculate Fn in linear time more easily by explicitly specifying the order of evaluation
// of the recurrence relation. Observe that we have removed all recursive calls! We evaluate the
// Fibonacci numbers from smallest to biggest and store all the results, so we know that we have
// Fi−1 and Fi−2 ready whenever we need to compute Fi. The linearity of this algorithm is now
// obvious. Each of the n values is simply computed as the sum of two integers, in O(n) total time
// and space.
//
// ANALYSIS:
// Time Complexity: O(n)
// Space Complexity: O(n)
//
pub fn fibonacci(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    let mut memo = vec![0; n + 1];
    memo[0] = 0;
    memo[1] = 1;

    for i in 2..=n {
        memo[i] = memo[i - 1] + memo[i - 2];
    }
    memo[n]
}

pub fn example() {
    let n = 20;
    let res = fibonacci(n);
    eprintln!("[{res}] fibonacci::bottom_up");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 0;
        let result = fibonacci(n);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let n = 1;
        let result = fibonacci(n);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let n = 2;
        let result = fibonacci(n);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_4() {
        let n = 3;
        let result = fibonacci(n);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_5() {
        let n = 20;
        let result = fibonacci(n);
        let expected = 6765;
        assert_eq!(result, expected);
    }
}
