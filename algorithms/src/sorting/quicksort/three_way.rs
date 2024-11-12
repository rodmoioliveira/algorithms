// REFERENCES:
//
// - [SW11] R. Sedgewick and K. Wayne, 2011. Algorithms. Addison-Wesley. ISBN 978-0-32-157351-3.

// [Wild2012, p.37]
// Algorithm 3. Quicksort with simple three-way partitioning from [SW11, page 299]. Note the
// resemblance to Algorithm 8; in fact YAROSLAVSKIY’s algorithm can be seen as improved version of
// this algorithm’s partitioning scheme.
pub fn _three_way<T: Ord>(a: &mut [T], left: usize, right: usize) {
    use std::cmp::Ordering;

    if left < right {
        let pivot_i = left;
        let mut lt = left;
        let mut gt = right + 1;
        let mut i = left + 1;

        while i < gt {
            let comp = a[i].cmp(&a[pivot_i]);
            match comp {
                Ordering::Less => {
                    a.swap(i, lt + 1);
                    i += 1;
                    lt += 1;
                }
                Ordering::Equal => {
                    i += 1;
                }
                Ordering::Greater => {
                    a.swap(i, gt - 1);
                    gt -= 1;
                }
            }
        }

        a.swap(pivot_i, lt);
        let checked_right = lt.checked_sub(1).unwrap_or_default();

        _three_way(a, left, checked_right);
        _three_way(a, gt, right);
    }
}

pub fn three_way<T: Ord>(a: &mut [T]) {
    let len = a.len();
    if len > 1 {
        _three_way(a, 0, len - 1);
    }
}

pub fn example() {
    let mut res = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
    three_way(&mut res);
    eprintln!("sorting::quicksort::three_way: {res:?}");
}

#[cfg(test)]
mod tests {
    use super::three_way;
    use crate::sorting::utils::*;

    #[test]
    fn basic() {
        let mut res = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
        let cloned = res.clone();
        three_way(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        let cloned = res.clone();
        three_way(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        three_way(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        let cloned = res.clone();
        three_way(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        let cloned = res.clone();
        three_way(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        let cloned = res.clone();
        three_way(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn large_elements() {
        let mut res = generate_random_vec(300000, 0, 1000000);
        let cloned = res.clone();
        log_timed("large elements test", || {
            three_way(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn nearly_ordered_elements() {
        let mut res = generate_nearly_ordered_vec(3000, 10);
        let cloned = res.clone();
        log_timed("nearly ordered elements test", || {
            three_way(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn repeated_elements() {
        let mut res = generate_repeated_elements_vec(1_000_000, 3);
        let cloned = res.clone();
        log_timed("repeated elements test", || {
            three_way(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }
}