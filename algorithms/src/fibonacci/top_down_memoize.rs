// REFERENCES:
// - [ADM] Skiena, Steven S. The Algorithm Design Manual. Third Edition, 2020, ISBN 978-3-030-54255-9.
//
// [ADM, 309]
// In fact, we can do much better. We can explicitly store (or cache) the results of each Fibonacci
// computation f(k) in a table data structure indexed by the parameter k — a technique also known
// as memoization.
//
//
//                                ┌──────────────────fib(6)───────────────────┐
//                                │                                           │
//                                │                                           │
//                    ┌─────────f(5) ────────┐                              f(4)
//                    │                      │
//                    │                      │
//              ┌───f(4)───┐               f(3)
//              │          │
//              │          │
//          ┌─f(3)─┐     f(2)
//          │      │
//          │      │
//      ┌─f(2)─┐ f(1)
//      │      │
//      │      │
//    f(1)   f(0)
//
//                                                Figure 10.2
//                        The recursion tree for computing Fibonacci numbers with caching.
//
//
// This cached version runs instantly up to the largest value that can fit in a long integer. The
// new recursion tree (Figure 10.2) explains why. There is no meaningful branching, because only
// the left-side calls do computation. The right-side calls find what they are looking for in the
// cache and immediately return.
//
// This general method of explicitly caching (or tabling) results from recursive calls to avoid
// recomputation provides a simple way to get most of the benefits of full dynamic programming. It
// is thus worth a careful look. In principle, such caching can be employed on any recursive
// algorithm. However, storing partial results would have done absolutely no good for such
// recursive algorithms as quicksort, backtracking, and depth-first search because all the recursive
// calls made in these algorithms have distinct parameter values. It doesn't pay to store something
// you will use once and never refer to again.
//
fn _fibonacci(n: usize, memo: &mut [Option<usize>]) -> usize {
    match memo[n] {
        Some(v) => v,
        None => match n {
            0 => 0,
            1 => 1,
            _ => {
                let v = _fibonacci(n - 1, memo) + _fibonacci(n - 2, memo);
                memo[n] = Some(v);
                v
            }
        },
    }
}

pub fn fibonacci(n: usize) -> usize {
    _fibonacci(n, &mut vec![None; n + 1])
}

pub fn example() {
    let n = 20;
    let res = fibonacci(n);
    eprintln!("[{res}] fibonacci::top_down_memoize");
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
