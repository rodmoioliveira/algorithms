// REFERENCES:
// - [ADM] Skiena, Steven S. The Algorithm Design Manual. Third Edition, 2020, ISBN 978-3-030-54255-9.
//
// [ADM, 308]
// The Fibonacci numbers were defined by the Italian mathematician Fibonacci in the thirteenth century
// to model the growth of rabbit populations. Rabbits breed, well, like rabbits. Fibonacci surmised
// that the number of pairs of rabbits born in a given month is equal to the number of pairs of
// rabbits born in each of the two previous months, starting from one pair of rabbits at the start.
// Thus, the number of rabbits born in the nth month is defined by the recurrence relation:
//
// Fn = Fn−1 + Fn−2
//
//
//                                ┌──────────────────fib(6)───────────────────┐
//                                │                                           │
//                                │                                           │
//                    ┌─────────f(5) ────────┐                    ┌─────────f(4)────────┐
//                    │                      │                    │                     │
//                    │                      │                    │                     │
//              ┌───f(4)───┐           ┌───f(3)───┐         ┌───f(3)───┐           ┌───f(2)───┐
//              │          │           │          │         │          │           │          │
//              │          │           │          │         │          │           │          │
//          ┌─f(3)─┐   ┌─f(2)─┐    ┌─f(2)─┐     f(1)    ┌─f(2)─┐     f(1)        f(1)       f(0)
//          │      │   │      │    │      │             │      │
//          │      │   │      │    │      │             │      │
//      ┌─f(2)─┐ f(1) f(1)  f(0) f(1)   f(0)          f(1)   f(0)
//      │      │
//      │      │
//    f(1)   f(0)
//
//                                                Figure 10.1
//                             The recursion tree for computing Fibonacci numbers.
//
//
// The course of execution for this recursive algorithm is illustrated by its recursion tree, as
// illustrated in Figure 10.1. This tree is evaluated in a depth-first fashion, as are all
// recursive algorithms. I encourage you to trace this example by hand to refresh your knowledge of
// recursion.
//
// Note that f(4) is computed on both sides of the recursion tree, and f(2) is computed no less
// than five times in this small example. The weight of all this redundancy becomes clear when you
// run the program. It took 4 minutes and 40 seconds for this program to compute f(50) on my
// laptop. You might well do it faster by hand using the algorithm below.
//
pub fn fibonacci(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn example() {
    let n = 20;
    let res = fibonacci(n);
    eprintln!("[{res}] fibonacci::recursive_naive");
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
