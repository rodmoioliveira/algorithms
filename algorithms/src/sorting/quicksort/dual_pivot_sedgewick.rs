// REFERENCES:
//
// - [162] R. Sedgewick. Quicksort. Reprint of the author's Ph. D. thesis, Garland Publishing, 1980.
// - [Sed75] R. Sedgewick, 1975. Quicksort. PhD Thesis, Stanford University.
// - [Wild2012] S. Wild. Java 7's Dual Pivot Quicksort. Master's Thesis Department of Computer Science, Technische Universität Kaiserslautern, 2012.
// - [Wild2016] S. Wild. Dual-Pivot Quicksort and Beyond: Analysis of Multiway Partitioning and Its Practical Potential. Doctoral's Thesis Department of Computer Science, Technische Universität Kaiserslautern, 2016.
//
// [Wild2012, p.52]
// Algorithm 7. Dual Pivot Quicksort with SEDGEWICK's partitioning. This algorithm appears as
// Program 5.1 in [Sed75].
//
// [Wild2016, p.142]
// Dual-pivot Quicksort was considered much earlier than the recency of YBB Quicksort might
// suggest. To the authors knowledge, Program 5.1 of Sedgewick [162] is the first full Quicksort
// implementation featuring two pivots. We reproduce it here as our Algorithm 6 and call it
// Sedgewick's (dual-pivot) partitioning.
//
// [Wild2016, p.142]
//
//   ┄ ┄┌─────────────────┬─────────────────┬─────────────────┬─────────────────┬─────────────────┐┄ ┄
//      │      < P1       │  P1 <= o <= P2  │        ?        │  P1 <= o <= P2  │       > P2      │
//   ┄ ┄└─────────────────┴─────────────────┴─────────────────┴─────────────────┴─────────────────┘┄ ┄
//    left               i1                 i                 j                 j1              right
//                       -->               -->               <--               <--
//
pub fn _dual_pivot_sedgewick<T: Copy + Ord>(a: &mut [T], left: usize, right: usize) {
    if left < right {
        if a[left] > a[right] {
            a.swap(left, right);
        }

        // pointers
        let mut i = left;
        let mut i1 = left;
        let mut j = right;
        let mut j1 = right;

        // pivot_l is the left pivot, and pivot_r is the right pivot.
        let pivot_l = a[left];
        let pivot_r = a[right];

        'outer: loop {
            i += 1;
            while a[i] <= pivot_r {
                if i >= j {
                    break 'outer;
                }
                if a[i] < pivot_l {
                    a[i1] = a[i];
                    i1 += 1;
                    a[i] = a[i1];
                }
                i += 1;
            }
            j -= 1;
            while a[j] >= pivot_l {
                if a[j] > pivot_r {
                    a[j1] = a[j];
                    j1 -= 1;
                    a[j] = a[j1];
                }
                if i >= j {
                    break 'outer;
                }
                j -= 1;
            }
            a[i1] = a[j];
            a[j1] = a[i];
            i1 += 1;
            j1 -= 1;
            a[i] = a[i1];
            a[j] = a[j1];
        }

        // Bring pivots to their appropriate positions.
        a[i1] = pivot_l;
        a[j1] = pivot_r;

        if i1 > 0 {
            _dual_pivot_sedgewick(a, left, i1 - 1);
        }
        _dual_pivot_sedgewick(a, i1 + 1, j1 - 1);
        _dual_pivot_sedgewick(a, j1 + 1, right);
    }
}

pub fn dual_pivot_sedgewick<T: Copy + Ord>(a: &mut [T]) {
    let len = a.len();
    if len > 1 {
        _dual_pivot_sedgewick(a, 0, len - 1);
    }
}

pub fn example() {
    let mut res = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
    dual_pivot_sedgewick(&mut res);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::dual_pivot_sedgewick;
    use crate::sorting::utils::*;

    #[test]
    fn basic() {
        let mut res = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
        let cloned = res.clone();
        dual_pivot_sedgewick(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        let cloned = res.clone();
        dual_pivot_sedgewick(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        dual_pivot_sedgewick(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        let cloned = res.clone();
        dual_pivot_sedgewick(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        let cloned = res.clone();
        dual_pivot_sedgewick(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        let cloned = res.clone();
        dual_pivot_sedgewick(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn large_elements() {
        let mut res = generate_random_vec(300000, 0, 1000000);
        let cloned = res.clone();
        log_timed("large elements test", || {
            dual_pivot_sedgewick(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn nearly_ordered_elements() {
        let mut res = generate_nearly_ordered_vec(3000, 10);
        let cloned = res.clone();
        log_timed("nearly ordered elements test", || {
            dual_pivot_sedgewick(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn repeated_elements() {
        let mut res = generate_repeated_elements_vec(1_000, 3);
        let cloned = res.clone();
        log_timed("repeated elements test", || {
            dual_pivot_sedgewick(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }
}
