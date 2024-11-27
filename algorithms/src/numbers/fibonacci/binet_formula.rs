// REFERENCES:
// - Dasdan, Ali. Twelve Simple Algorithms to Compute Fibonacci Numbers. https://arxiv.org/pdf/1803.07199
// - Luo, Long. 9 Fibonacci Algorithms: The Most Complete Solutions. https://www.longluo.me/blog/2022/01/29/fibonacci-sequence/
// - Nayuki. Fast Fibonacci algorithms. https://www.nayuki.io/page/fast-fibonacci-algorithms
// - [ADM] Skiena, Steven S. The Algorithm Design Manual. Third Edition, 2020, ISBN 978-3-030-54255-9.
//
// ANALYSIS:
// Time Complexity: O(log n)
// Space Complexity: O(1)
//
pub fn fibonacci(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    let n_f32 = n as f32;
    let sqrt_5: f32 = f32::sqrt(5_f32);
    let fib_n: f32 =
        f32::powf((1.0 + sqrt_5) / 2_f32, n_f32) - f32::powf((1_f32 - sqrt_5) / 2.0, n_f32);
    f32::round(fib_n / sqrt_5) as usize
}

pub fn example() {
    let n = 20;
    let res = fibonacci(n);
    eprintln!("{} {:?}", module_path!(), res);
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
