// REFERENCES:
// - Dasdan, Ali. Twelve Simple Algorithms to Compute Fibonacci Numbers. https://arxiv.org/pdf/1803.07199
// - Luo, Long. 9 Fibonacci Algorithms: The Most Complete Solutions. https://www.longluo.me/blog/2022/01/29/fibonacci-sequence/
// - Nayuki. Fast Fibonacci algorithms. https://www.nayuki.io/page/fast-fibonacci-algorithms
// - [ADM] Skiena, Steven S. The Algorithm Design Manual. Third Edition, 2020, ISBN 978-3-030-54255-9.
//
// ANALYSIS:
// Time Complexity: O(log n)
// Space Complexity: O(log n), if we consider the function call stack size, otherwise O(1)
//
pub fn fibonacci(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    let mut _n = n;
    let mut l_matrix = [[1, 1], [1, 0]];
    let res = matrix_pow(&mut l_matrix, &mut (_n - 1));
    res[0][0]
}

fn matrix_mult(l_matrix: [[usize; 2]; 2], r_matrix: [[usize; 2]; 2]) -> [[usize; 2]; 2] {
    let mut res = [[0, 0], [0, 0]];
    for i in 0..2 {
        for j in 0..2 {
            res[i][j] = l_matrix[i][0] * r_matrix[0][j] + l_matrix[i][1] * r_matrix[1][j];
        }
    }
    res
}

// [https://www.longluo.me/blog/2022/01/29/fibonacci-sequence/#solution-7-fast-matrix-poweroptimized-method-6]
// The key here is to compute `l_matrix^n` using the successive square method. Using this
// algorithm, `l_matrix^n` is computed in O(log n) time (Note that for a fixed matrix size, the
// matrix muliplication algorithm takes a constant amount of time).
fn matrix_pow(l_matrix: &mut [[usize; 2]; 2], n: &mut usize) -> [[usize; 2]; 2] {
    let mut res = [[1, 0], [0, 1]];
    while *n > 0 {
        if (*n & 1) == 1 {
            res = matrix_mult(res, *l_matrix);
        }
        *n >>= 1;
        *l_matrix = matrix_mult(*l_matrix, *l_matrix);
    }
    res
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
        let res = fibonacci(n);
        let expected = 0;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let n = 1;
        let res = fibonacci(n);
        let expected = 1;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let n = 2;
        let res = fibonacci(n);
        let expected = 1;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let n = 3;
        let res = fibonacci(n);
        let expected = 2;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_5() {
        let n = 20;
        let res = fibonacci(n);
        let expected = 6765;
        assert_eq!(res, expected);
    }
}
