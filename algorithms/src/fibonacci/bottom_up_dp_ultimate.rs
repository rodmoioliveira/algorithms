// REFERENCES:
// - [ADM] Skiena, Steven S. The Algorithm Design Manual. Third Edition, 2020, ISBN 978-3-030-54255-9.
//
// [ADM, 312]
// More careful study shows that we do not need to store all the intermediate values for the entire
// period of execution. Because the recurrence depends on two arguments, we only need to retain the
// last two values we have seen.
//
pub fn fibonacci(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    let mut memo = [0; 3];
    memo[0] = 0;
    memo[1] = 1;

    for _ in 2..=n {
        memo[2] = memo[0] + memo[1];
        memo[0] = memo[1];
        memo[1] = memo[2];
    }
    memo[2]
}

pub fn example() {
    let n = 20;
    let res = fibonacci(n);
    eprintln!("[{res}] fibonacci::bottom_up_dp_ultimate");
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
