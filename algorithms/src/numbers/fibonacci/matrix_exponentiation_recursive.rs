// REFERENCES:
// - Dasdan, Ali. Twelve Simple Algorithms to Compute Fibonacci Numbers. https://arxiv.org/pdf/1803.07199
// - Luo, Long. 9 Fibonacci Algorithms: The Most Complete Solutions. https://www.longluo.me/blog/2022/01/29/fibonacci-sequence/
// - Nayuki. Fast Fibonacci algorithms. https://www.nayuki.io/page/fast-fibonacci-algorithms
// - [ADM] Skiena, Steven S. The Algorithm Design Manual. Third Edition, 2020, ISBN 978-3-030-54255-9.
//
// ANALYSIS:
// Time Complexity: O(n)
// Space Complexity: O(1)
//
pub fn fibonacci(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    let mut l_matrix = [[1, 1], [1, 0]];
    matrix_pow(&mut l_matrix, n - 1);
    l_matrix[0][0]
}

fn matrix_mult(l_matrix: &mut [[usize; 2]; 2], r_matrix: [[usize; 2]; 2]) {
    let x = l_matrix[0][0] * r_matrix[0][0] + l_matrix[0][1] * r_matrix[1][0];
    let y = l_matrix[0][0] * r_matrix[0][1] + l_matrix[0][1] * r_matrix[1][1];
    let z = l_matrix[1][0] * r_matrix[0][0] + l_matrix[1][1] * r_matrix[1][0];
    let w = l_matrix[1][0] * r_matrix[0][1] + l_matrix[1][1] * r_matrix[1][1];

    l_matrix[0][0] = x;
    l_matrix[0][1] = y;
    l_matrix[1][0] = z;
    l_matrix[1][1] = w;
}

fn matrix_pow(l_matrix: &mut [[usize; 2]; 2], n: usize) {
    for _ in 2..=n {
        matrix_mult(l_matrix, [[1, 1], [1, 0]]);
    }
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
