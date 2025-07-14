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
// [https://www.longluo.me/blog/2022/01/29/fibonacci-sequence/#solution-8-ologn-time]
// if n is even then k = n/2 giving F(n) = (2F(k - 1) + F(k)) * F(k)
// else n is odd then (n + 1)/2  giving  F(n) = F(k)*F(k) + F(k - 1)*F(k - 1)
//
// [https://www.nayuki.io/page/fast-fibonacci-algorithms]
// These identities can be extracted from the matrix exponentiation algorithm. In a sense, this
// algorithm is the matrix exponentiation algorithm with the redundant calculations removed. It
// should be a constant factor faster than matrix exponentiation, but the asymptotic time
// complexity is still the same.
//
fn _fibonacci(n: usize, memo: &mut [usize]) -> usize {
    if n < 2 {
        return n;
    }

    if memo[n] != 0 {
        return memo[n];
    }

    let k = if (n & 1) == 1 { n.div_ceil(2) } else { n / 2 };
    memo[n] = if (n & 1) == 1 {
        _fibonacci(k, memo) * _fibonacci(k, memo)
            + _fibonacci(k - 1, memo) * _fibonacci(k - 1, memo)
    } else {
        (2 * _fibonacci(k - 1, memo) + _fibonacci(k, memo)) * _fibonacci(k, memo)
    };
    memo[n]
}

pub fn fibonacci(n: usize) -> usize {
    _fibonacci(n, &mut vec![0; n + 1])
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
