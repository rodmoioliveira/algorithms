// REFERENCES:
//
// - [Hoa1961a] C. A. R. Hoare, 1961. Algorithm 63: Partition. Communications of the ACM, 4(7):321. doi:10.1145/366622.366642.
// - [Hoa1961b] C. A. R. Hoare, 1961. Algorithm 65: Find. Communications of the ACM, 4(7):321–322. doi:10.1145/366622.366647.
// - [SF1996] R. Sedgewick and P. Flajolet, 1996. An Introduction to the Analysis of Algorithms. Addison-Wesley-Longman. ISBN 978-0-201-40009-0.
// - [Sed1975] R. Sedgewick, 1975. Quicksort. PhD Thesis, Stanford University.
// - [Sed1978] R. Sedgewick, 1978. Implementing Quicksort programs. Communications of the ACM, 21(10):847–857. doi:10.1145/359619.359631.

// [Wild2012, p.26]
// Algorithm 1. Classic Quicksort implementation by SEDGEWICK as given and discussed in detail in
// [Sed1975, Sed1978]. We take the rightmost element as pivot instead of the leftmost, as it is done in
// Program 1.2 of [SF1996]. Partitioning is done as follows: Two pointers i and j scan the array from
// left and right until they hit an element that does not belong in this subfile. Then the elements
// A[i] and A[j] are exchanged. This crossing pointers technique dates back to HOARE’s original
// formulation of Quicksort [Hoa1961a].
pub fn _classic_hoare<T: Ord>(a: &mut [T], left: usize, right: usize) {
    if left < right {
        let pivot_i = right;
        let mut i = left;
        let mut j = right - 1;

        loop {
            while a[i] < a[pivot_i] {
                i += 1;
            }
            while j > 0 && a[j] > a[pivot_i] {
                j -= 1;
            }

            if i >= j {
                break;
            }

            if a[i] != a[j] {
                a.swap(i, j);
            } else {
                i += 1;
                j -= 1;
            }
        }
        a.swap(i, pivot_i);

        if i > 0 {
            _classic_hoare(a, left, i - 1);
        }
        _classic_hoare(a, i + 1, right);
    }
}

pub fn classic_hoare<T: Ord>(a: &mut [T]) {
    let len = a.len();
    if len > 1 {
        _classic_hoare(a, 0, len - 1);
    }
}

pub fn example() {
    let mut res = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
    classic_hoare(&mut res);
    eprintln!("sorting::quicksort::classic_hoare: {res:?}");
}

#[cfg(test)]
mod tests {
    use super::classic_hoare;
    use crate::sorting::utils::*;

    #[test]
    fn basic() {
        let mut res = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
        let cloned = res.clone();
        classic_hoare(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        let cloned = res.clone();
        classic_hoare(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        classic_hoare(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        let cloned = res.clone();
        classic_hoare(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        let cloned = res.clone();
        classic_hoare(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        let cloned = res.clone();
        classic_hoare(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn large_elements() {
        let mut res = generate_random_vec(300000, 0, 1000000);
        let cloned = res.clone();
        log_timed("large elements test", || {
            classic_hoare(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn nearly_ordered_elements() {
        let mut res = generate_nearly_ordered_vec(3000, 10);
        let cloned = res.clone();
        log_timed("nearly ordered elements test", || {
            classic_hoare(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn repeated_elements() {
        let mut res = generate_repeated_elements_vec(1_000_000, 3);
        let cloned = res.clone();
        log_timed("repeated elements test", || {
            classic_hoare(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }
}
