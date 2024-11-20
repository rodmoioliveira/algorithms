// REFERENCES:
//
// - [Ben1999] Jon L. Bentley. Programming Pearls. Addison-Wesley, second edition, 1999.
// - [CLRS2009] T. H. Cormen, C. E. Leiserson, R. L. Rivest and C. Stein, 2009. Introduction to Algorithms (3rd ed.). MIT Press. ISBN 978-0-262-03384-8.
// - [Hoa1962] C. A. R. Hoare. Quicksort. The Computer Journal, 5(1):10315, 1962.
// - [Wild2012] S. Wild. Java 7’s Dual Pivot Quicksort. Master’s Thesis Department of Computer Science, Technische Universität Kaiserslautern, 2012.
// - [Wild2016] S. Wild. Dual-Pivot Quicksort and Beyond: Analysis of Multiway Partitioning and Its Practical Potential. Doctoral’s Thesis Department of Computer Science, Technische Universität Kaiserslautern, 2016.

// [CLRS2009, p.204]
// Quicksort was invented by [Hoa1962], and his version of PARTITION appears in Problem 7-1.
// [Ben1999, p. 117] attributes the PARTITION procedure given in Section 7.1 to N. Lomuto.

// [Wild2012, p.35]
// Algorithm 2. Quicksort variant from [CLRS2009, Chapter 7]. It uses a particularly simple partitioning
// scheme, which is not based on HOARE’s crossing pointers technique.
pub fn _lomuto_clrs<T: Ord>(a: &mut [T], left: usize, right: usize) {
    if left < right {
        let pivot_i = right;
        let mut i = left;

        for j in left..right {
            if a[j] <= a[pivot_i] {
                a.swap(i, j);
                i += 1;
            }
        }
        a.swap(i, right);

        if i > 0 {
            _lomuto_clrs(a, left, i - 1);
        }
        _lomuto_clrs(a, i + 1, right);
    }
}

pub fn lomuto_clrs<T: Ord>(a: &mut [T]) {
    let len = a.len();
    if len > 1 {
        _lomuto_clrs(a, 0, len - 1);
    }
}

pub fn example() {
    let mut res = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
    lomuto_clrs(&mut res);
    eprintln!("{res:?} sorting::quicksort::lomuto_clrs");
}

#[cfg(test)]
mod tests {
    use super::lomuto_clrs;
    use crate::sorting::utils::*;

    #[test]
    fn basic() {
        let mut res = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
        let cloned = res.clone();
        lomuto_clrs(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        let cloned = res.clone();
        lomuto_clrs(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        lomuto_clrs(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        let cloned = res.clone();
        lomuto_clrs(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        let cloned = res.clone();
        lomuto_clrs(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        let cloned = res.clone();
        lomuto_clrs(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn large_elements() {
        let mut res = generate_random_vec(300000, 0, 1000000);
        let cloned = res.clone();
        log_timed("large elements test", || {
            lomuto_clrs(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn nearly_ordered_elements() {
        let mut res = generate_nearly_ordered_vec(3000, 10);
        let cloned = res.clone();
        log_timed("nearly ordered elements test", || {
            lomuto_clrs(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn repeated_elements() {
        let mut res = generate_repeated_elements_vec(1_000, 3);
        let cloned = res.clone();
        log_timed("repeated elements test", || {
            lomuto_clrs(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }
}
