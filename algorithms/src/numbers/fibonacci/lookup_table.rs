// REFERENCES:
// - Dasdan, Ali. Twelve Simple Algorithms to Compute Fibonacci Numbers. https://arxiv.org/pdf/1803.07199
// - Luo, Long. 9 Fibonacci Algorithms: The Most Complete Solutions. https://www.longluo.me/blog/2022/01/29/fibonacci-sequence/
// - Nayuki. Fast Fibonacci algorithms. https://www.nayuki.io/page/fast-fibonacci-algorithms
// - [ADM] Skiena, Steven S. The Algorithm Design Manual. Third Edition, 2020, ISBN 978-3-030-54255-9.
//
// ANALYSIS:
// Time Complexity: O(1)
// Space Complexity: O(1)
//
pub fn fibonacci(n: usize) -> usize {
    if n > 47 {
        panic!("n should be less than 48");
    }

    let table: [usize; 48] = [
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269,
        2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155,
        165580141, 267914296, 433494437, 701408733, 1134903170, 1836311903, 2971215073,
    ];
    table[n]
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
