// REFERENCES:
// - Harold Abelson and Gerald Jay Sussman with Julie Sussman. Structure and Interpretation of Computer Programs. Chapter 1: 1.2.4 Exponentiation
//
fn _fibonacci(a: usize, b: usize, p: usize, q: usize, n: usize) -> usize {
    match n == 0 {
        true => b,
        false => match n.is_multiple_of(2) {
            true => {
                let p_prime = p.pow(2) + q.pow(2);
                let q_prime = q.pow(2) + (2 * p * q);
                _fibonacci(a, b, p_prime, q_prime, n / 2)
            }
            false => {
                let a_prime = (b * q) + (a * q) + (a * p);
                let b_prime = (b * p) + (a * q);
                _fibonacci(a_prime, b_prime, p, q, n - 1)
            }
        },
    }
}

pub fn fibonacci(n: usize) -> usize {
    _fibonacci(1, 0, 0, 1, n)
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
